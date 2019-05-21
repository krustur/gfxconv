use crate::error::ErrorKind;
use crate::iff::chunks::form_ilbm_chunk::FormIlbmChunk;

pub mod chunks;
pub mod raw;

#[derive(Debug)]
pub struct IffFile {
    pub ilbm: FormIlbmChunk,
}

impl IffFile {


    pub fn parse_iff_buffer(buffer: &[u8]) -> Result<IffFile, ErrorKind> {
        if buffer.len() < 12 {
            return Err(ErrorKind::FileTooShort);
        }

        let iff_file = IffFile {
            ilbm: FormIlbmChunk::parse_form_ilbm_buffer(buffer)?,
        };

        Ok(iff_file)
    }
}