use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{editor::Editor, file::save_file};

/// This function is for handling keyboard keys
/// The return type is for telling if the editor should quit (true) or not (false)
pub fn handle_key(
    key: KeyEvent,
    editor: &mut Editor,
    visible_line_count: usize,
    path: &Path,
) -> bool {
    match key.code {
        KeyCode::Char('q') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                true
            } else {
                false
            }
        }
        KeyCode::Char('s') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match save_file(path, &editor.lines.join("\n")) {
                    Ok(()) => {}
                    Err(err) => editor.status = err.to_string(),
                };
                return false;
            } else {
                false
            }
        }
        KeyCode::Up => {
            if editor.cursor.line > 0 {
                editor.cursor.line -= 1;
                let line_len = editor.line_chars()[editor.cursor.line];
                editor.cursor.col = editor.cursor.col.min(line_len);
            };
            false
        }
        KeyCode::Down => {
            if editor.cursor.line + 1 < editor.lines.len() {
                editor.cursor.line += 1;
                let line_len = editor.line_chars()[editor.cursor.line];
                editor.cursor.col = editor.cursor.col.min(line_len);
            };
            false
        }
        KeyCode::Left => {
            editor.cursor.col = editor.cursor.col.saturating_sub(1);
            false
        }
        KeyCode::Right => {
            if editor.cursor.col
                < editor
                    .line_chars()
                    .get(editor.cursor.line)
                    .copied()
                    .unwrap_or(0)
            {
                editor.cursor.col += 1;
            };
            false
        }
        KeyCode::PageUp => {
            editor.cursor.line = editor.cursor.line.saturating_sub(visible_line_count);
            false
        }
        KeyCode::PageDown => {
            editor.cursor.line =
                (editor.cursor.line + visible_line_count).min(editor.lines.len().saturating_sub(1));
            false
        }
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                return false;
            }

            let byte_index = editor.lines[editor.cursor.line]
                .char_indices()
                .nth(editor.cursor.col)
                .map(|(i, _)| i)
                .unwrap_or(editor.lines[editor.cursor.line].len());

            editor.lines[editor.cursor.line].insert(byte_index, c);
            editor.cursor.col += 1;
            false
        }
        _ => false,
    }
}
