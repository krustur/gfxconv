use crate::ErrorKind;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::path;
use std::str;

// impl BufferReader{
pub fn get_str(buffer: &[u8], pos: usize) -> Result<&str, ErrorKind> {
    let chunk_id = str::from_utf8(&buffer[pos..pos + 4]);
    let chunk_id2 = match chunk_id {
        Ok(x) => x,
        Err(err) => {
            let err_msg = fmt::format(format_args!(
                "{}: [{:X}] [{:X}] [{:X}] [{:X}]",
                err,
                buffer[pos + 0],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ));
            return Err(ErrorKind::UnknownChunk(err_msg));
        }
    };
    // println!("group_id {:?}", chunk_id);

    Ok(chunk_id2)
}

pub fn get_u32(buffer: &[u8], pos: usize) -> Result<u32, ErrorKind> {
    let slize = &buffer[pos..pos + 4];
    let mut array: [u8; 4] = [0; 4];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 4], u32>(array).to_be() };
    let value = value as u32;

    Ok(value)
}

pub fn get_u16(buffer: &[u8], pos: usize) -> Result<u16, ErrorKind> {
    let slize = &buffer[pos..pos + 2];
    let mut array: [u8; 2] = [0; 2];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 2], u16>(array).to_be() };
    let value = value as u16;

    Ok(value)
}

pub fn get_i16(buffer: &[u8], pos: usize) -> Result<i16, ErrorKind> {
    let slize = &buffer[pos..pos + 2];
    let mut array: [u8; 2] = [0; 2];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 2], i16>(array).to_be() };
    let value = value as i16;

    Ok(value)
}

pub fn get_u8(buffer: &[u8], pos: usize) -> Result<u8, ErrorKind> {
    // let slize = &buffer[pos..pos + 2];
    // let mut array: [u8; 2] = [0; 2];
    // array.copy_from_slice(slize);
    // let value = unsafe { mem::transmute::<[u8; 2], get_u8>(array).to_be() };
    // let value = value as get_u8;

    let value = buffer[pos];
    Ok(value)
}
// }
