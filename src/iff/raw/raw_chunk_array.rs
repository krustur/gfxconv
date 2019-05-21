use crate::error::ErrorKind;
use crate::iff::raw::raw_chunk::RawChunk;

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

impl<'a> RawChunkArray<'a> {
    pub fn get_first(&mut self) -> Result<Option<RawChunk<'a>>, ErrorKind> {
        self.pos = 0;
        let raw_chunk_result = self.get_next();
        raw_chunk_result
    }

    pub fn get_next(&mut self) -> Result<Option<RawChunk<'a>>, ErrorKind> {
        if self.pos >= self.buffer.len() {
            return Ok(None);
        }

        let raw_chunk = RawChunk::from(self.buffer, self.pos)?;

        self.pos += 8;
        self.pos += ((raw_chunk.size as usize) + 1) & 0xfffffffffffffffe;

        Ok(Some(raw_chunk))
    }
}
