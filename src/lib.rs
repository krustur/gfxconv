// mod model;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    NotFORMGroupChunk,
}

pub struct IffFile {
    pub width: u32,
}

pub fn read_iff_file(file_path: std::path::PathBuf) -> Result<IffFile, ErrorKind> {
    let mut f = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(ErrorKind::IoError(error)),
    };
    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(file) => file,
        Err(error) => return Err(ErrorKind::IoError(error)),
    };

    parse_iff_buffer(&buffer) // {
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<IffFile, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let group_id = std::str::from_utf8(&buffer[0..4]);
    let group_id = match group_id {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::NotFORMGroupChunk),
    };
    if group_id != "FORM" {
        return Err(ErrorKind::NotFORMGroupChunk);
    }
    // let hej2 = std::str::from_utf8(hej);
    println!("group_id {:?}", group_id);

    let gfx = IffFile { width: 320 };

    Ok(gfx)
}
