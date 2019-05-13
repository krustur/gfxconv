use crate::buffer_reader;
use crate::ErrorKind;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::path;
use std::str;

// #[derive(Debug)]
pub struct RawChunk<'a> {
    pub id: &'a str,
    pub size: usize,
    buffer: &'a [u8],
}

impl<'a> fmt::Debug for RawChunk<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} [{:?} ... ]",
            self.id,
            self.size,
            &self.buffer[0..8]
        )
    }
}

impl<'a> RawChunk<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<RawChunk, ErrorKind> {
        if buffer.len() < 8 {
            return Err(ErrorKind::ChunkTooShort);
        };

        let size = buffer_reader::get_u32(buffer, 4)? as usize;
        if size == 0 {
            return Err(ErrorKind::ZeroSizeChunk);
        }

        Ok(RawChunk {
            id: buffer_reader::get_str(buffer, 0)?,
            size: size,
            buffer: buffer,
        })
    }

    // pub fn first_from(buffer: &'a [u8]) -> Result<RawChunk, ErrorKind> {
    //     let raw_chunk = RawChunk { buffer: buffer };
    //     let id = raw_chunk.get_id()?;
    //     println!("id: {:?}", id);
    //     let size = raw_chunk.get_u32(4)? as usize;
    //     println!("size: {:?}", size);

    //     let raw_chunk = RawChunk {
    //         buffer: &buffer[0..(8 + size)],
    //     };
    //     Ok(raw_chunk)
    // }

    // fn extract_id(&self) -> Result<&'a str, ErrorKind> {
    //     self.get_str(0)
    // }

    // fn extract_size(&self) -> Result<u32, ErrorKind> {
    //     self.get_u32(4)
    // }

    pub fn get_str(&self, pos: usize) -> Result<&'a str, ErrorKind> {
        let val = buffer_reader::get_str(self.buffer, pos)?;
        Ok(val)
    }
    // println!("group_id {:?}", chunk_id);

    pub fn get_u32(&self, pos: usize) -> Result<u32, ErrorKind> {
        let val = buffer_reader::get_u32(self.buffer, pos)?;
        Ok(val)
    }
}
