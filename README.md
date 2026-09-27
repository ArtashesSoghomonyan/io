# IO
Terminal based text editor written in Rust. This text editor is not modal like vim

## Installation
### Macos/Linux
```bash
curl --proto '=https' -sSf https://raw.githubusercontent.com/ArtashesSoghomonyan/io/refs/heads/main/scripts/install.sh | sh
```

### Windows (Only x86_64)
You can install manually with `.exe` file at [this link](https://github.com/ArtashesSoghomonyan/io/releases/latest/)

## Keybindings
Currently the keybindings are not customizable, meaning that you can't add new keybindings or change the existing ones

| Key | Action |
| --- | ------ |
| `Ctrl + S` | Save |
| `Ctrl + Q` | Quit |
| `↑` `↓` `←` `→` | Navigation |
| `PageUp` `↑ + Shift` | Page Up |
| `PageDown` `↓ + Shift` | Page Down |

## Settings
Below are the settings that you can use for customizing the editor. Settings are located in the home directory of a user in the `io.config.toml` file

### Editor Section

| Setting | Type |
| --- | --- |
| `display_cursor_position` | Boolean |
| `enable_line_numbers` | Boolean |
