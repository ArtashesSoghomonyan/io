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
    let mut stdout = io::stdout();
    let _guard = TerminalGuard::new()?;
    let mut top= 0usize;
    let mut cursor_line = 0usize;
    let mut cursor_col = 0usize;

    loop {
        let (width, height) = terminal::size()?;
        let view = height.saturating_sub(1) as usize;
        let width = width.max(1) as usize;

        for row in 0..view {
            let text: String = lines
                .get(top + row)
                .map(|line| line.chars().take(width.saturating_sub(1)).collect())
                .unwrap_or_default(); // NOTE: chars(), not display columns
            queue!(stdout, cursor::MoveTo(0, row as u16), terminal::Clear(terminal::ClearType::CurrentLine), Print(text))?;
        }
        let status = format!(" {} lines | Ctrl+Q quit ", lines.len());
        queue!(stdout, cursor::MoveTo(0, view as u16), terminal::Clear(terminal::ClearType::CurrentLine), Print(status))?;
        
        // This part was AI generated btw...
        let caret_row = cursor_line.saturating_sub(top as usize).min(view.saturating_sub(1));
        let caret_col = cursor_col.min(width.saturating_sub(1));
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
                    }
                },
                KeyCode::Down => {
                    if cursor_line + 1 < lines.len() {
                        cursor_line += 1;
                    }
                },
                KeyCode::Left => {
                    if cursor_col > 0 {
                        cursor_col -= 1;
                    }
                },
                KeyCode::Right => {
                    // We are checking if there are no lines because the program panics
                    //   on keydown::right when the file is empty
                    if lines.len() != 0 && cursor_col < lines[cursor_line].chars().count() - 1 {
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

        if cursor_line < top {
            top = cursor_line;
        } else if cursor_line >= top + view {
            top = cursor_line - view + 1;
        }
    }
}
