// mod model;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    UnknownChunk,
    ZeroSizeChunk,
    FoundMultipleChunksInBuffer,
    UnsupportedFormType,
    UnknownFormType,
}

pub trait IffChunk {
    fn get_id(&self) -> &str;
    fn get_children(&self) -> &Vec<Box<IffChunk>>;
}

// #[derive(Debug)]
pub struct IffFormChunk {
    pub id: String,
    children: Vec<Box<IffChunk>>,
}

impl IffFormChunk {
    pub fn new(id: String) -> IffFormChunk {
        IffFormChunk {
            children: Vec::new(),
            id: id,
        }
    }
}

impl IffChunk for IffFormChunk {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_children(&self) -> &Vec<Box<IffChunk>> {
        &self.children
    }
}

impl std::fmt::Debug for IffFormChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn read_iff_file(file_path: std::path::PathBuf) -> Result<Vec<Box<IffChunk>>, ErrorKind> {
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
    // println!("ending: {:?}", ending);
    ending
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<Vec<Box<IffChunk>>, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let iff_chunks = parse_chunk_buffer(buffer)?;

    Ok(iff_chunks)
}

fn parse_chunk_buffer(buffer: &[u8]) -> Result<Vec<Box<IffChunk>>, ErrorKind> {
    let mut pos = 0usize;
    let mut iff_chunks: Vec<Box<IffChunk>>;

    iff_chunks = Vec::new();

    while pos < buffer.len() {
        let chunk_id = get_chunk_id(buffer, pos + 0)?;
        let chunk_size = get_chunk_size(buffer, pos + 4)?;

        if chunk_size == 0 {
            return Err(ErrorKind::ZeroSizeChunk);
        }

        match chunk_id {
            "FORM" => {
                let form_type = get_chunk_id(buffer, pos + 8)?;
                println!("FORM type {}", form_type);
                match form_type {
                    "ILBM" => println!("ILBM"),
                    "ANIM" => {
                        println!("ANIM");
                        return Err(ErrorKind::UnsupportedFormType);
                    }
                    _ => return Err(ErrorKind::UnknownFormType),
                }

                let iff_form_chunk = Box::new(IffFormChunk::new(chunk_id.to_string()));
                iff_chunks.push(iff_form_chunk);

                // iff_file.width = 320;

                let form_buffer = &buffer[pos + 12..pos + 12 + chunk_size - 4];
                println!("form_buffer len: {}", form_buffer.len());
                let mut form_pos = 0usize;
                // while form_pos < buffer.len() {
                let form_chunk_id = get_chunk_id(form_buffer, form_pos + 0)?;
                let form_chunk_size = get_chunk_size(form_buffer, form_pos + 4)?;
                println!("form_chunk_id: {}", form_chunk_id);
                println!("form_chunk_size: {}", form_chunk_size);

                form_pos += 8;
                form_pos += chunk_size;
                // }
                // parse_iff_chunk()
            }

            _ => return Err(ErrorKind::UnknownChunk),
        }

        pos += 8;
        pos += chunk_size;
        // if pos < buffer.len() {
        //     return Err(ErrorKind::FoundMultipleChunksInBuffer);
    }

    Ok(iff_chunks)
}

fn get_chunk_id(buffer: &[u8], pos: usize) -> Result<&str, ErrorKind> {
    let chunk_id = std::str::from_utf8(&buffer[pos..pos + 4]);
    let chunk_id = match chunk_id {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::UnknownChunk),
    };
    println!("group_id {:?}", chunk_id);

    Ok(chunk_id)
}

fn get_chunk_size(buffer: &[u8], pos: usize) -> Result<usize, ErrorKind> {
    let chunk_size_slize = &buffer[pos..pos + 4];
    let mut chunk_size_array: [u8; 4] = [0; 4];
    chunk_size_array.copy_from_slice(chunk_size_slize);
    let chunk_size = unsafe { std::mem::transmute::<[u8; 4], u32>(chunk_size_array).to_be() };
    let chunk_size = chunk_size as usize;
    println!("chunk_size {:?}", chunk_size);

    Ok(chunk_size)
}
