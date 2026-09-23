//! This module is for working with files and directories

use std::{
    fs::{ self, File, OpenOptions },
    io,
    path::{ Component, Path },
};

use colored::Colorize;
use inquire::Confirm;
use posix_portable_filename::PortableFilename;

/// Checks that a path is non-empty and that every one of its components is a
/// valid POSIX portable filename.
/// `/` is allowed as a separator (`src/editor.rs`), `.` is allowed as a no-op
/// component, but `..` is rejected so a path can never walk out of the
/// directory the user pointed us at.
fn is_valid_path(path: &Path) -> bool {
    // Note: This function was generated with AI, I have no idea how it works
    let mut components = path.components().peekable();

    if components.peek().is_none() {
        return false; // empty path
    }

    components.all(|component| match component {
        Component::Normal(name) => name
            .to_str()
            .is_some_and(|name| PortableFilename::new(name).is_ok()),
        Component::CurDir | Component::RootDir => true,
        Component::ParentDir | Component::Prefix(_) => false,
    })
}

/// This function checks if a file is writeable or not
fn can_write(path: &Path) -> bool {
    OpenOptions::new()
        .write(true)
        .open(path)
        .is_ok()
}

/// This function creates new files
fn create_file(filename: &str) -> Result<String, io::Error> {
    // Checking if filename is valid and can be real (for example not .. or -123)
    if !is_valid_path(Path::new(filename)) {
        let err_message = format!("Error: {filename} is not a valid filename.");
        return Err(io::Error::new(io::ErrorKind::InvalidFilename, err_message.red().to_string()));
    }

    match fs::write(filename, "") {
        Ok(_) => return Ok(String::new()),
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
            let err_message = format!("Error: Permission denied to create a file.");
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, err_message.red().to_string()));
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let err_message = format!("Error: File not found");
            return Err(io::Error::new(io::ErrorKind::InvalidFilename, err_message.red().to_string()));
        },
        Err(error) if error.kind() == io::ErrorKind::OutOfMemory => {
            let err_message = format!("Error: not enough memory to save the file.");
            return Err(io::Error::new(io::ErrorKind::OutOfMemory, err_message.red().to_string()));
        },
        Err(error) if error.kind() == io::ErrorKind::StorageFull => {
            let err_message = format!("Error: not enough storage space to save the file.");
            return Err(io::Error::new(io::ErrorKind::StorageFull, err_message.red().to_string()));
        },
        Err(error) => {
            return Err(io::Error::new(error.kind(), error));
        }
    };
}

/// This function tries to open a file with filename
/// If it fails to open it, because.
///   A) It doesn't exist -> It will ask the user with yes/no prompt if they would like to create one.
///   B) If we don't have permissions to read/write on os it will write an error and suggest to chmod. 
pub fn open_file(filename: &str) -> Result<String, io::Error> {
    // Checking if filename is valid and can be real (for example not .. or -123)
    if !is_valid_path(Path::new(filename)) {
        let err_message = format!("Error: {filename} is not a valid filename.");
        return Err(io::Error::new(io::ErrorKind::InvalidFilename, err_message.red().to_string()));
    }

    let path = Path::new(filename);

    if path.exists() {
        // Checking if the file is a file and not directory, link etc.
        if !path.is_file() {
            let err_message = format!("Error: {filename} is not a file.");
            return Err(io::Error::new(io::ErrorKind::InvalidData, err_message.red().to_string()));
        }

        // Checking if the file is readable
        if !File::open(path).is_ok() {
            let err_message = format!("Error: File is not readable.\nHint: Try to do `$ chmod +r {filename}`");
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, err_message.red().to_string()));
        }

        // Checking if the file is writeable
        if !can_write(&path) {
            let err_message = format!("Error: File is not writeable.\nHint: Try to do `$ chmod +w {filename}`");
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, err_message.red().to_string()));
        }

        let content = fs::read_to_string(filename)
            .map_err(|err| io::Error::new(err.kind(), format!("couldn't read {filename:?}: {err}").red().to_string()))?;
        return Ok(content);
    } else {
        let message = format!("There is no file called {filename}, would you like to create one?");
        let answer = Confirm::new(&message)
            .with_default(true)
            .prompt();

        match answer {
            Ok(true) => return create_file(filename),
            Ok(false) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Ok, bye!")),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Error. Couldn't get the answer".red().to_string())),
        }
    }
}
