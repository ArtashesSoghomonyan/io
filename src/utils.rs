//! This module is responsible for different small utility features

use std::io;

use colored::Colorize;
use inquire::Confirm;

use crate::file::create_file;

pub fn prompt_create_new_file(filename: &str) -> io::Result<()> {
    let message = format!("There is no file called {filename}, would you like to create one?");
    let answer = Confirm::new(&message).with_default(true).prompt();

    match answer {
        Ok(true) => create_file(filename),
        Ok(false) => Err(io::Error::new(io::ErrorKind::InvalidData, "Ok, bye!")),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error. Couldn't get the answer".red().to_string(),
            ));
        }
    }
}
