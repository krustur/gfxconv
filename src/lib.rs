// mod model;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
}

pub struct RawGraphics {
    pub width: u32,
}

pub fn read_iff_file(file_path: std::path::PathBuf) -> Result<RawGraphics, ErrorKind> {
    let mut f = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(ErrorKind::IoError(error)),
    };
    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(file2) => file2,
        Err(error2) => return Err(ErrorKind::IoError(error2)),
    };

    parse_iff_buffer(&buffer) // {
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<RawGraphics, ErrorKind> {
    let hej = &buffer[0..4];
    let hej2 = std::str::from_utf8(hej);
    println!("header {:?}", hej2);

    let gfx = RawGraphics { width: 1123 };

    Ok(gfx)
}
