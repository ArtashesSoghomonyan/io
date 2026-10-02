//! This module is responsible for working with arguments and starting the editor

use std::{io, path::Path};

use clap::Parser;

use crate::{
    SETTINGS, editor::{FileEditor, display_file}, file::{SupportedFiletype, open_file, path_filetype}, settings::load_settings, terminal::TerminalGuard, utils::prompt_create_new_file,
};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
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

        match path_filetype(filename) {
            Ok(SupportedFiletype::File) => {
                let _guard = TerminalGuard::new()?;
                let mut editor = FileEditor::new(filename)?;
                editor.render.start()?;
            }
            Ok(SupportedFiletype::Directory) => {
                println!("Directories are not supported yet...");
                return Ok(())
            }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::InvalidInput => {
                        eprintln!("{}", error.to_string());
                        return Err(error)
                    }
                    io::ErrorKind::InvalidFilename => {
                        match prompt_create_new_file(filename) {
                            Ok(_) => return Ok(()),
                            Err(error) => {
                                eprintln!("{error}");
                                std::process::exit(1);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
