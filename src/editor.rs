use std::{
    fs,
    io::{self, Write},
    path::Path
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print},
    terminal,
    queue
};

use crate::terminal_guard::TerminalGuard;

pub fn open_file(filename: &Path) -> io::Result<()> {
    let content = fs::read_to_string(filename)
        .map_err(|err| io::Error::new(err.kind(), format!("couldn't read {filename:?}: {err}")))?;

    let lines: Vec<&str> = content.lines().collect();
    let line_chars: Vec<usize> = lines.iter().map(|line| line.chars().count()).collect();
    let mut stdout = io::stdout();
    let _guard = TerminalGuard::new()?;
    let mut top= 0usize;
    let mut cursor_line = 0usize;
    let mut cursor_col = 0usize;

    loop {
        let (width, height) = terminal::size()?;
        let view = height.saturating_sub(1) as usize;
        let width = width.max(1) as usize;
        let wrap = width.saturating_sub(1).max(1);

        let mut first_row: Vec<usize> = Vec::with_capacity(lines.len() + 1);
        let mut next = 0usize;
        for &len in &line_chars {
            first_row.push(next);
            next += len.div_ceil(wrap).max(1); // an empty line still occupies one row
        }
        first_row.push(next); // sentinel = total visual rows
        let total_rows = next;

        for row in 0..view {
            let vis = top + row;
            let text: String = if vis < total_rows {
                let line_idx = first_row.partition_point(|&start| start <= vis) - 1;
                let offset = (vis - first_row[line_idx]) * wrap;
                lines[line_idx].chars().skip(offset).take(wrap).collect()
            } else {
                String::new()
            };
            queue!(stdout, cursor::MoveTo(0, row as u16), terminal::Clear(terminal::ClearType::CurrentLine), Print(text))?;
        }
        let status = format!(" {} lines | Ctrl+Q quit ", lines.len());
        queue!(stdout, cursor::MoveTo(0, view as u16), terminal::Clear(terminal::ClearType::CurrentLine), Print(status))?;
        
        // This part was AI generated btw...
        let caret_vis = first_row.get(cursor_line).copied().unwrap_or(0) + cursor_col / wrap;
        let caret_col = cursor_col % wrap; // column inside the visual row
        let caret_row = caret_vis.saturating_sub(top).min(view.saturating_sub(1));
        queue!(stdout, cursor::MoveTo(caret_col as u16, caret_row as u16))?;

        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => match key.code {
                KeyCode::Char('q')
                    if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if cursor_line > 0 {
                        cursor_line -= 1;
                        let line_len = line_chars[cursor_line];
                        cursor_col = cursor_col.min(line_len.saturating_sub(1));
                    }
                },
                KeyCode::Down => {
                    if cursor_line + 1 < lines.len() {
                        cursor_line += 1;
                        let line_len = line_chars[cursor_line];
                        cursor_col = cursor_col.min(line_len.saturating_sub(1));
                    }
                },
                KeyCode::Left => cursor_col = cursor_col.saturating_sub(1),
                KeyCode::Right => {
                    if cursor_col + 1 < line_chars.get(cursor_line).copied().unwrap_or(0) {
                        cursor_col += 1;
                    }
                },
                KeyCode::PageUp => cursor_line = cursor_line.saturating_sub(view),
                KeyCode::PageDown => {
                    cursor_line = (cursor_line + view).min(lines.len().saturating_sub(1))
                }
                _ => {}
            },
            _ => {} // Resize etc. is redrawn at the top of the loop
        }

        if caret_vis < top {
            top = caret_vis;
        } else if caret_vis >= top + view {
            top = caret_vis - view + 1;
        }
    }
}
