// mod model;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    UnknownChunk,
    UnknownFormType,
}

#[derive(Debug)]
pub struct IffFile {
    pub width: u32,
}

pub fn read_iff_file(file_path: std::path::PathBuf) -> Result<IffFile, ErrorKind> {
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

    let ending = parse_iff_buffer(&buffer);
    println!("ending: {:?}", ending);
    ending
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<IffFile, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let mut iff_file = IffFile { width: 0 };

    parse_iff_chunk(buffer, &mut iff_file)?;

    Ok(iff_file)
}

fn parse_iff_chunk(buffer: &Vec<u8>, iff_file: &mut IffFile) -> Result<(), ErrorKind> {
    let mut pos = 0usize;
    while pos < buffer.len() {
        let chunk_id = get_chunk_id(buffer, pos + 0)?;
        let chunk_size = get_chunk_size(buffer, pos + 4)?;

        match chunk_id {
            "FORM" => {
                let form_type = get_chunk_id(buffer, pos + 8)?;
                println!("FORM type {}", form_type);
                match form_type {
                    "ILBM" => println!("ILBM"),
                    // "ANIM" => println!("anim"),
                    _ => return Err(ErrorKind::UnknownFormType),
                }

                // parse_iff_chunk()
            }

            _ => return Err(ErrorKind::UnknownChunk),
        }

        // TODO: Break on chunk_size 0 (ErrorKind::ZeroSizeChunk)
        pos += 8;
        pos += chunk_size;
    }

    (*iff_file).width = 320;

    Ok(())
}

fn get_chunk_id(buffer: &Vec<u8>, pos: usize) -> Result<&str, ErrorKind> {
    let chunk_id = std::str::from_utf8(&buffer[pos..pos + 4]);
    let chunk_id = match chunk_id {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::UnknownChunk),
    };
    println!("group_id {:?}", chunk_id);

    Ok(chunk_id)
}

fn get_chunk_size(buffer: &Vec<u8>, pos: usize) -> Result<usize, ErrorKind> {
    let chunk_size_slize = &buffer[pos..pos + 4];
    let mut chunk_size_array: [u8; 4] = [0; 4];
    chunk_size_array.copy_from_slice(chunk_size_slize);
    let chunk_size = unsafe { std::mem::transmute::<[u8; 4], u32>(chunk_size_array).to_be() };
    let chunk_size = chunk_size as usize;
    println!("chunk_size {:?}", chunk_size);

    Ok(chunk_size)
}
