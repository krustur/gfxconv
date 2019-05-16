use crate::buffer_reader;
use crate::ErrorKind;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::ops::Range;
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
    pub fn from(buffer: &'a [u8], pos: usize) -> Result<RawChunk, ErrorKind> {
        if buffer.len() < 8 {
            return Err(ErrorKind::ChunkTooShort);
        };

        let id = buffer_reader::get_str(buffer, pos + 0)?;
        let size = buffer_reader::get_u32(buffer, pos + 4)? as usize;
        if size == 0 {
            return Err(ErrorKind::ZeroSizeChunk);
        }

        // if size != buffer.len() - 8 {
        //     println!("Invalid chunk size: {:?} - {:?}", size, buffer.len() - 8);
        //     return Err(ErrorKind::InvalidChunkSize);
        // }

        Ok(RawChunk {
            id: id,
            size: size,
            buffer: &buffer[pos..],
        })
    }

    pub fn get_str(&self, pos: usize) -> Result<&'a str, ErrorKind> {
        let val = buffer_reader::get_str(self.buffer, pos)?;
        Ok(val)
    }
    // println!("group_id {:?}", chunk_id);

    pub fn get_u32(&self, pos: usize) -> Result<u32, ErrorKind> {
        let val = buffer_reader::get_u32(self.buffer, pos)?;
        Ok(val)
    }

    pub fn get_u16(&self, pos: usize) -> Result<u16, ErrorKind> {
        let val = buffer_reader::get_u16(self.buffer, pos)?;
        Ok(val)
    }

    pub fn get_i16(&self, pos: usize) -> Result<i16, ErrorKind> {
        let val = buffer_reader::get_i16(self.buffer, pos)?;
        Ok(val)
    }

    pub fn get_u8(&self, pos: usize) -> Result<u8, ErrorKind> {
        let val = buffer_reader::get_u8(self.buffer, pos)?;
        Ok(val)
    }

    pub fn get_slice_to_end(&self, pos: usize) -> &'a [u8] {
        let slice = &self.buffer[pos..];
        slice
    }

    pub fn get_slice(&self, range: Range<usize>) -> &'a [u8] {
        let slice = &self.buffer[range];
        slice
    }
}
