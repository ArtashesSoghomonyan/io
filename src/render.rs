use std::{
    io::{self, Write},
    path::PathBuf,
};

use crossterm::{
    cursor, event::{self, Event}, execute, style::Print, terminal,
};

pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

pub struct Render {
    pub path: PathBuf,

    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub caret: Cursor,
    pub width: usize,
    pub height: usize,
    pub top_line: usize,

}

impl Render {
    pub fn new(path: PathBuf, content: String) -> io::Result<Self> {
        let lines = if content == "" {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        let (width, height) = terminal::size()?;

        Ok(Self {
            path,
            
            lines,
            top_line: 0,
            cursor: Cursor { line: 0, col: 0 },
            caret: Cursor { line: 0 , col: 0 },

            width: width as usize,
            height: height as usize,
        })
    }

    pub fn start(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        loop {
            let (width, height) = terminal::size()?;
            self.width = width as usize;
            self.height = height as usize;

            let mut line_count = 0;
            execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

            'line_loop: for line in self.lines[self.top_line..].iter() {
                let visual_lines = if line.is_empty() {
                    vec![String::new()]
                } else {
                    to_chunks(&line, self.width)
                };

                for display_line in visual_lines.iter() {
                    execute!(
                        stdout,
                        cursor::MoveTo(0, line_count as u16),
                        Print(display_line)
                    )?;

                    if line_count < self.height {
                        line_count += 1;
                    } else {
                        break 'line_loop;
                    }
                }
            }

            execute!(stdout, cursor::MoveTo(self.caret.col as u16, self.caret.line as u16))?;

            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        event::KeyCode::Char('q') => return Ok(()),
                        event::KeyCode::Up => {
                            
                        },
                        event::KeyCode::Down => {
                            
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    }
}

pub fn to_chunks(s: &String, chunk_size: usize) -> Vec<String> {
    let mut chunks = vec![];
    let mut chars = s.chars();

    loop {
        let chunk: String = chars.by_ref().take(chunk_size).collect();
        if chunk.is_empty() {
            break;
        }
        chunks.push(chunk);
    }

    chunks
}

