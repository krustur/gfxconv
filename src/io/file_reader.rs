use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn read_file(file_path: &PathBuf) -> Result<Vec<u8>, Error> {
    println!("file_path {:?}", file_path);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    Ok(buffer)
}