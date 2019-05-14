use crate::raw_chunk::*;

pub struct RawChunkArray<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> RawChunkArray<'a> {
    pub fn from(buffer: &[u8]) -> RawChunkArray {
        RawChunkArray {
            buffer: buffer,
            pos: 0,
        }
    }
}

impl<'a> Iterator for RawChunkArray<'a> {
    type Item = RawChunk<'a>;

    fn next(&mut self) -> Option<RawChunk<'a>> {
        if self.pos >= self.buffer.len() {
            return None;
        }

        let raw_chunk = match RawChunk::from(self.buffer, self.pos) {
            Ok(chunk) => chunk,
            Err(_) => return None,
        };

        // println!("pos: {:?}", self.pos);
        self.pos += 8;
        self.pos += ((raw_chunk.size as usize) + 1) & 0xfffffffffffffffe;

        Some(raw_chunk)
    }
}
