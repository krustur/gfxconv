use crate::iff::raw::raw_chunk::RawChunk;
use crate::error::ErrorKind;

// BmhdChunk
#[derive(Debug)]
pub struct BmhdChunk {
    pub id: String,
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub number_of_planes: u8,
    pub masking: u8,
    pub compression: u8,
    pub transparent_color_number: u16,
    pub x_aspect: u8,
    pub y_aspect: u8,
    pub page_width: i16,
    pub page_height: i16,
}

impl BmhdChunk{
    pub fn get_bmhd_chunk(raw_chunk: &RawChunk) -> Result<BmhdChunk, ErrorKind> {
        let chunk = BmhdChunk {
            id: String::from("BMHD"),
            width: raw_chunk.get_u16(8 + 0)?,
            height: raw_chunk.get_u16(8 + 2)?,
            x: raw_chunk.get_i16(8 + 4)?,
            y: raw_chunk.get_i16(8 + 6)?,
            number_of_planes: raw_chunk.get_u8(8 + 8)?,
            masking: raw_chunk.get_u8(8 + 9)?,
            compression: raw_chunk.get_u8(8 + 10)?,
            // UBYTE pad1
            transparent_color_number: raw_chunk.get_u16(8 + 12)?,
            x_aspect: raw_chunk.get_u8(8 + 14)?,
            y_aspect: raw_chunk.get_u8(8 + 15)?,
            page_width: raw_chunk.get_i16(8 + 16)?,
            page_height: raw_chunk.get_i16(8 + 18)?,
        };

        Ok(chunk)
    }
}