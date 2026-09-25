use std::fs;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    #[serde(default)]
    pub editor: EditorSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: EditorSettings::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct EditorSettings {
    pub line_numbers: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            line_numbers: true,
        }
    }
}

pub fn load_settings() -> Settings {
    let home = dirs::home_dir().expect("Could not find home directory");
    let path = home.join("io.config.toml");
    
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return Settings::default(),
    };

    match toml::from_str(&contents) {
        Ok(data) => data,
        // TODO: Maybe display in error screen later.
        Err(_) => Settings::default(),
    }
}
