use crate::raw_chunk::*;

pub struct RawChunkArray<'a> {
    buffer: &'a [u8],
    pos: usize,
    // curr: u32,
    // next: u32,
}

impl<'a> RawChunkArray<'a> {
    pub fn from(buffer: &[u8]) -> RawChunkArray {
        RawChunkArray {
            buffer: buffer,
            pos: 0,
        }
    }
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for RawChunkArray<'a> {
    type Item = RawChunk<'a>;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<RawChunk<'a>> {
        if self.pos >= self.buffer.len() {
            return None;
        }

        let raw_chunk = match RawChunk::from(self.buffer) {
            Ok(chunk) => chunk,
            Err(_) => return None,
        };
        // .unwrap_or_else(|err| {
        // return None;
        // });
        //TODO: Unexpected chunk ends

        // let chunk_id = get_chunk_id(buffer, pos + 0)?;
        // let chunk_size = get_u32(buffer, pos + 4)? as usize;
        self.pos += raw_chunk.size as usize + 8;

        Some(raw_chunk)

        // let new_next = self.curr + self.next;

        // self.curr = self.next;
        // self.next = new_next;

        // // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // // will never return `None`, and `Some` is always returned.
        // Some(self.curr)
    }
}
