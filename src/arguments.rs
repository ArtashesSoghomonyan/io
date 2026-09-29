//! This module is responsible for working with arguments and starting the editor

use std::{io, path::Path};

use clap::Parser;

use crate::{
    SETTINGS,
    editor::display_file,
    file::{open_file, path_filetype, SupportedFiletype},
    settings::load_settings,
    utils::prompt_create_new_file,
};

#[derive(Parser, Debug)]
struct Arguments {
    filename: Option<String>,

    #[arg(short, long, long_help="Print version (can also be used with '-v')")]
    version: bool,
}

/// This function is running the editor depending on the arguments
/// The program can be used in following ways
///   1. io -> displays version and usage
///   2. version -> displays version number
///   3. <filename> -> opens io in file editor mode
///   4. <directory> -> opens io in directory editor mode (NOTE: currently not done)
pub fn run() -> io::Result<()> {
    let arguments = Arguments::parse();

    // case (io --version)
    if arguments.version {
        println!("io v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    
    // case (io <filename>/<directory>)
    if let Some(filename) = arguments.filename.as_deref() {
        SETTINGS.set(load_settings()).unwrap();
        let path = Path::new(filename);

        match path_filetype(filename) {
            Ok(SupportedFiletype::File) => {
                let content = open_file(filename)?;
                return display_file(path, &content);
            }
            Ok(SupportedFiletype::Directory) => {
                println!("Directories are not supported yet...");
                return Ok(())
            }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::InvalidInput => {
                        eprintln!("{}", error);
                        return Err(error)
                    }
                    io::ErrorKind::InvalidFilename => {
                        return prompt_create_new_file(filename)
                    }
                    _ => {}
                }
            }
        }
    }

    // case else
    println!("io text editor (v{}) (\"less --help\" for help)", env!("CARGO_PKG_VERSION"));
    println!("usage: io <filename>");
    Ok(())
}
