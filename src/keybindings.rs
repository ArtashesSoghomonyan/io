use colored::Colorize;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{editor::Editor, file::save_file};

/// This function is for handling keyboard keys
/// The return type is for telling if the editor should quit (true) or not (false)
pub fn handle_key(key: KeyEvent, editor: &mut Editor, visible_line_count: usize) -> bool {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if editor.asking_to_quit {
                return true;
            }

            if editor.is_changed {
                editor.status_message = String::from(
                    "Would you like to quit without saving? (Ctrl + q)"
                        .red()
                        .to_string(),
                );
                editor.asking_to_quit = true;
                return false;
            } else {
                return true;
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            editor.asking_to_quit = false;
            match save_file(&editor.path, &editor.lines.join("\n")) {
                Ok(()) => {
                    editor.is_changed = false;
                    editor.status_message = String::from("Saved");
                }
                Err(err) => editor.status_message = err.to_string().red().to_string(),
            };
            false
        }
        KeyCode::Up => {
            // Shift + up = page up
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                editor.cursor.line = editor.cursor.line.saturating_sub(visible_line_count);
                return false;
            }

            if editor.cursor.line > 0 {
                editor.cursor.line -= 1;
                let line_len = editor.line_chars()[editor.cursor.line];
                editor.cursor.col = editor.cursor.col.min(line_len);
            };
            false
        }
        KeyCode::Down => {
            // Shift + down = page down
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                editor.cursor.line = (editor.cursor.line + visible_line_count)
                    .min(editor.lines.len().saturating_sub(1));
                return false;
            }

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
        KeyCode::Backspace => {
            if editor.lines.len() == 0 {
                return false;
            }

            if editor.lines[editor.cursor.line].len() == 0 && editor.cursor.line != 0 {
                editor.lines.remove(editor.cursor.line);
                editor.cursor.line -= 1;
                editor.cursor.col = editor.lines[editor.cursor.line].len();
            }

            if editor.lines[editor.cursor.line].len() > 0 {
                if editor.cursor.col == 0 {
                    let new_cursor_col = editor.lines[editor.cursor.line - 1].len();
                    let current_line = editor.lines.remove(editor.cursor.line);
                    editor.lines[editor.cursor.line - 1].push_str(&current_line);
                    editor.cursor.line -= 1;
                    editor.cursor.col = new_cursor_col;
                } else {
                    editor.lines[editor.cursor.line].remove(editor.cursor.col - 1);
                    editor.cursor.col -= 1;
                }
            }

            editor.is_changed = true;
            editor.status_message = editor.status_bar();
            return false;
        }
        KeyCode::Enter => {
            let reminder: String = editor.lines[editor.cursor.line]
                .chars()
                .skip(editor.cursor.col)
                .collect();

            editor.lines[editor.cursor.line] = editor.lines[editor.cursor.line]
                .chars()
                .take(editor.cursor.col)
                .collect();

            editor.lines.insert(editor.cursor.line + 1, reminder);
            editor.cursor.line += 1;
            editor.cursor.col = 0;

            editor.is_changed = true;
            editor.status_message = String::new();
            return false;
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
            editor.is_changed = true;
            editor.status_message = String::new();
            false
        }
        _ => false,
    }
}
