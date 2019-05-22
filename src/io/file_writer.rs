use std::path::PathBuf;
use std::io::{Error, Write};
use std::fs::File;

pub fn write_file(file_path: &PathBuf, buffer: &[u8]) -> Result<(), Error> {
    println!("file_path {:?}", file_path);

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    match file.write_all(buffer) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    Ok(())
}