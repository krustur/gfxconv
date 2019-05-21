use std::fs::File;
use std::io::Read;
use std::path;

use crate::error::ErrorKind;
use crate::iff::chunks::form_ilbm_chunk::FormIlbmChunk;

pub mod chunks;
pub mod raw;

#[derive(Debug)]
pub struct IffFile {
    pub ilbm: FormIlbmChunk,
}

impl IffFile {
    pub fn read_iff_file(file_path: path::PathBuf) -> Result<IffFile, ErrorKind> {
        println!("file_path {:?}", file_path);

        let mut f = match File::open(file_path) {
            Ok(file) => file,
            Err(error) => return Err(ErrorKind::IoError(error)),
        };
        let mut buffer = Vec::new();
        match f.read_to_end(&mut buffer) {
            Ok(file) => file,
            Err(error) => return Err(ErrorKind::IoError(error)),
        };

        let iff_file = IffFile::parse_iff_buffer(&buffer)?;

        Ok(iff_file)
    }

    pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<IffFile, ErrorKind> {
        if buffer.len() < 12 {
            return Err(ErrorKind::FileTooShort);
        }

        let iff_file = IffFile {
            ilbm: FormIlbmChunk::parse_form_ilbm_buffer(buffer)?,
        };

        Ok(iff_file)
    }
}