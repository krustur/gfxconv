use std::io::{Error, Read};
use std::path::PathBuf;
use std::fs::File;

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
//    let iff_file = IffFile::parse_iff_buffer(&buffer)?;

//    Ok(iff_file)
}