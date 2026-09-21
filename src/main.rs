mod terminal_guard;
mod editor;

use std::path::Path;

use clap::Parser;
use inquire::Confirm;

use editor::open_file;

#[derive(Parser, Debug)]
struct Arguments {
    filename: String,

    #[arg(short, long)]
    version: bool,
}

/// This is the entry point of our program.
/// A filename should be specified and we check if it exists or not
/// If the file does not exist we ask the user with a yes/no inquire prompt if he/she wants to create one
/// If the input was not a file (for example a directory) we display an error
fn main() {
    let arguments = Arguments::parse();
    let path = Path::new(&arguments.filename);

    if path.exists() {
        if path.is_file() {
            let _ = open_file(path);
        } else {
            println!("Error. {:?} is not a file.", path);
        }
    } else {
        let message = format!("There is no file {:?}, would you like to add one?", path);
        let answer = Confirm::new(&message)
            .with_default(true)
            .prompt();

        match answer {
            Ok(true) => println!("yes"),
            Ok(false) => println!("no"),
            Err(_) => println!("Error. Couldn't get the answer"),
        }
    }
}
