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
        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Char('c')
                    if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => top = top.saturating_sub(1),
                KeyCode::Down if top + view < lines.len() => top += 1,
                KeyCode::PageUp => top = top.saturating_sub(view),
                KeyCode::PageDown => top = (top + view).min(lines.len().saturating_sub(view)),
                _ => {}
            },
            _ => {} // Resize etc. is redrawn at the top of the loop
        }
    }
}
