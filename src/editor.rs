use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use crossterm::{
    cursor,
    event::{self, Event},
    queue,
    style::Print,
    terminal,
};

use crate::{
    SETTINGS,
    keybindings::handle_key,
    settings::{EditorSettings, Settings},
    terminal::TerminalGuard,
};

#[derive(Clone)]
pub struct EditorCursor {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Editor {
    pub path: PathBuf,

    pub lines: Vec<String>,
    pub cursor: EditorCursor,
    pub top: usize,
    pub width: u16,
    pub height: u16,

    pub settings: EditorSettings,
    pub status_message: String,
    pub is_changed: bool,
}

impl Editor {
    pub fn line_chars(&self) -> Vec<usize> {
        self.lines.iter().map(|line| line.chars().count()).collect()
    }

    pub fn line_number_margin(&self) -> usize {
        if self.settings.enable_line_numbers {
            self.lines.len().to_string().len() + 1
        } else {
            0
        }
    }

    pub fn status_bar(&self) -> String {
        let space_left: usize;
        let left_part: String;

        if self.settings.display_cursor_position {
            // 1000, 1000 -> 8 + 1 = 9
            space_left = self.width.saturating_sub(9) as usize;
        } else {
            space_left = self.width as usize;
        }

        if !self.status_message.is_empty() {
            left_part = self
                .status_message
                .chars()
                .take(space_left.saturating_sub(3))
                .collect();
        } else {
            left_part = self
                .path
                .to_string_lossy()
                .chars()
                .take(space_left.saturating_sub(3))
                .collect();
        }

        if self.settings.display_cursor_position {
            let cursor = format!("{}, {}", self.cursor.line + 1, self.cursor.col + 1);
            format!("{left_part:<w$}{cursor:>9}", w = space_left)
        } else {
            left_part
        }
    }
}

pub fn display_file(path: &Path, content: &String) -> io::Result<()> {
    let mut stdout = io::stdout();
    let _guard: TerminalGuard = TerminalGuard::new()?;
    let status_bar_height = 1;
    let mut editor = Editor {
        path: path.to_path_buf(),

        lines: content.lines().map(|s| s.to_string()).collect(),
        cursor: EditorCursor { line: 0, col: 0 },
        top: 0,
        width: 0,
        height: 0,

        settings: SETTINGS
            .get()
            .unwrap_or(&Settings::default())
            .editor
            .clone(),
        status_message: String::new(),
        is_changed: false,
    };

    loop {
        let (width, height) = terminal::size()?;
        editor.width = width;
        editor.height = height;
        let visible_line_count = editor.height.saturating_sub(status_bar_height) as usize;
        let width = editor.width.max(1) as usize;
        let visible_row_count = width.saturating_sub(1 + editor.line_number_margin()).max(1);

        let mut first_row: Vec<usize> = Vec::with_capacity(editor.lines.len() + 1);
        let mut next: usize = 0;
        for &len in &editor.line_chars() {
            first_row.push(next);
            next += len.div_ceil(visible_row_count).max(1); // an empty line still occupies one row
        }
        first_row.push(next); // sentinel = total visual rows
        let total_rows = next;

        for row in 0..visible_line_count {
            let vis = editor.top + row;
            let (gutter, text): (String, String) = if vis < total_rows {
                let line_idx = first_row.partition_point(|&start| start <= vis) - 1;
                let offset = (vis - first_row[line_idx]) * visible_row_count;
                let text: String = editor.lines[line_idx]
                    .chars()
                    .skip(offset)
                    .take(visible_row_count)
                    .collect();
                let gutter = if editor.line_number_margin() == 0 {
                    String::new()
                } else if offset == 0 {
                    format!("{:>w$} ", line_idx + 1, w = editor.line_number_margin() - 1)
                } else {
                    " ".repeat(editor.line_number_margin())
                };
                (gutter, text)
            } else {
                (String::new(), String::new())
            };
            queue!(
                stdout,
                cursor::MoveTo(0, row as u16),
                terminal::Clear(terminal::ClearType::CurrentLine),
                Print(gutter),
                Print(text)
            )?;
        }
        queue!(
            stdout,
            cursor::MoveTo(0, visible_line_count as u16),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(&editor.status_bar())
        )?;

        // This part was AI generated btw...
        let caret_vis = first_row.get(editor.cursor.line).copied().unwrap_or(0)
            + editor.cursor.col / visible_row_count;
        let caret_col = (editor.line_number_margin() + editor.cursor.col % visible_row_count)
            .min(width.saturating_sub(1));
        let caret_row = caret_vis
            .saturating_sub(editor.top)
            .min(visible_line_count.saturating_sub(1));
        queue!(stdout, cursor::MoveTo(caret_col as u16, caret_row as u16))?;

        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => {
                match handle_key(key, &mut editor, visible_line_count) {
                    true => return Ok(()),
                    false => {}
                }
            }
            _ => {}
        }

        if caret_vis < editor.top {
            editor.top = caret_vis;
        } else if caret_vis >= editor.top + visible_line_count {
            editor.top = caret_vis - visible_line_count + 1;
        }
    }
}
