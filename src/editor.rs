use std::{
    fs::File,
    io::Read,
    path::Path
};

pub fn open_file(filename: &Path) {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Error. Something went wrong, couldn't open {:?}", filename);
            return;
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => println!("{}", content),
        Err(_) => println!("Error. Couldn't read {:?}", filename)
    }
}
