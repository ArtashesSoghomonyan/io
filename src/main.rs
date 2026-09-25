mod editor;
mod file;
mod keybindings;
mod settings;
mod terminal;

use std::{path::Path, sync::OnceLock};

use clap::Parser;

use editor::display_file;
use file::open_file;
use settings::{Settings, load_settings};

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Parser, Debug)]
struct Arguments {
    filename: Option<String>,

    #[arg(short, long)]
    version: bool,
}

/// This is the entry point of our program.
/// A filename should be specified and we check if it exists or not
/// If the file does not exist we ask the user with a yes/no inquire prompt if he/she wants to create one
/// If the input was not a file (for example a directory) we display an error
fn main() {
    let arguments = Arguments::parse();

    if arguments.version {
        println!("io v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    SETTINGS.set(load_settings()).unwrap();

    if let Some(filename) = arguments.filename.as_deref() {
        let content = match open_file(filename) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };
        if let Err(error) = display_file(Path::new(filename), &content) {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
