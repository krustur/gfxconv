use crate::error::ErrorKind;
use std::fmt;
use std::mem;
use std::str;

// impl BufferReader{
pub fn get_chunk_id(buffer: &[u8], pos: usize) -> Result<&str, ErrorKind> {
    let chunk_id = get_string(&buffer[pos..pos + 4]);

    let chunk_id = match chunk_id {
        Ok(x) => x,
        Err(err) => {
            let err_msg = fmt::format(format_args!(
                "{:?}: [{:X}] [{:X}] [{:X}] [{:X}]",
                err,
                buffer[pos + 0],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ));
            return Err(ErrorKind::UnknownChunk(err_msg));
        }
    };

    Ok(chunk_id)
}

pub fn get_string(buffer: &[u8]) -> Result<&str, ErrorKind> {
    let chunk_id = str::from_utf8(&buffer);

    let chunk_id = match chunk_id {
        Ok(x) => x,
        Err(err) => {
            return Err(ErrorKind::UnparseableString(err));
        }
    };

    Ok(chunk_id)
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
    let value = buffer[pos];
    Ok(value)
}

pub fn get_i8(buffer: &[u8], pos: usize) -> Result<i8, ErrorKind> {
    let slize = &buffer[pos..pos + 1];
    let mut array: [u8; 1] = [0; 1];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 1], i8>(array).to_be() };//buffer[pos];
    Ok(value)
}