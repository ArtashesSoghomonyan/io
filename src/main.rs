mod arguments;
mod editor;
mod file;
mod keybindings;
mod settings;
mod terminal;
mod utils;

use std::{io, sync::OnceLock};

use settings::{Settings};
use arguments::run;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

fn main() -> io::Result<()> {
    return run();
}
