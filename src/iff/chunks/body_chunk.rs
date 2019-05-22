use crate::error::ErrorKind;
use crate::iff::chunks::bmhd_chunk::BmhdChunk;
use crate::iff::raw::raw_chunk::RawChunk;

pub struct BodyChunk {
//    pub pixels: Vec<u8>,
    pub interleaved_bitmap_data: Vec<u8>,
    pub raw_buffer: Vec<u8>,
}

impl BodyChunk {
    pub fn get_body_chunk(raw_chunk: &RawChunk, bmhd: &Option<BmhdChunk>) -> Result<BodyChunk, ErrorKind> {
        let bmhd = match bmhd {
            None => return Err(ErrorKind::BmhdNotYetSet),
            Some(b) => b,
        };

//        let no_pixels = bmhd.width as usize * bmhd.height as usize;

        let mut actual_number_of_planes = bmhd.number_of_planes as usize;
        if bmhd.masking == 1 {
            actual_number_of_planes += 1;
        }
        let bytes_per_row_per_plane = ((bmhd.width as usize + 15) & 0xfffffff0) / 8;
        let bytes_per_row_all_planes = bytes_per_row_per_plane * actual_number_of_planes;
        let interleaved_size = bytes_per_row_all_planes * bmhd.height as usize;

        let mut chunk = BodyChunk {
            raw_buffer: vec![0; raw_chunk.size],
            interleaved_bitmap_data: vec![0; interleaved_size],
//            pixels: vec![0; no_pixels],
        };

        &chunk
            .raw_buffer
            .clone_from_slice(raw_chunk.get_slice(8..raw_chunk.size + 8));

        let mut pos: usize = 0;
        let mut target_pos: usize = 0;
        let mut _written_bytes: usize = 0;
        while pos < raw_chunk.size {
            let n = raw_chunk.get_i8(pos)?;
            pos += 1;
            if n == -128
            {
                return Err(ErrorKind::IlbmNoOp);
            } else if n < 0 {
                let new_n = -n;
                for _i in 0..new_n + 1 {
                    chunk.interleaved_bitmap_data[target_pos] = raw_chunk.get_u8(pos)?;
                    target_pos += 1;
                }
                _written_bytes += new_n as usize + 1;
                pos += 1;
            } else {
                for _i in 0..n + 1 {
                    chunk.interleaved_bitmap_data[target_pos] = raw_chunk.get_u8(pos)?;
                    target_pos += 1;
                    pos += 1;
                }
                _written_bytes += n as usize + 1;
            }
        }

        Ok(chunk)
    }
}