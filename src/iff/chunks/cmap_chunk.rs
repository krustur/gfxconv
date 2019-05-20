use std::{fmt, cmp};
use crate::iff::raw::raw_chunk::RawChunk;
use crate::error::ErrorKind;
use crate::common::col_rgb_u8::ColRgbU8;

// #[derive(Debug)]
pub struct CmapChunk {
    pub rgb: Vec<ColRgbU8>,
}

impl fmt::Debug for CmapChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cnt = cmp::min(4, self.rgb.len());

        for i in 0..cnt {
            write!(f, "{:?} ", self.rgb[i])?;
        }
        Ok(())
    }
}

impl CmapChunk{
    pub fn get_cmap_chunk(raw_chunk: &RawChunk) -> Result<CmapChunk, ErrorKind> {
        let no_colors = (raw_chunk.size) / 3;
        let mut chunk = CmapChunk {
            rgb: vec![ColRgbU8 { r: 0, g: 0, b: 0 }; no_colors],
        };

        for i in 0..no_colors {
            chunk.rgb[i].r = raw_chunk.get_u8(8 + i * 3 + 0)?;
            chunk.rgb[i].g = raw_chunk.get_u8(8 + i * 3 + 1)?;
            chunk.rgb[i].b = raw_chunk.get_u8(8 + i * 3 + 2)?;
        }

        Ok(chunk)
    }
}