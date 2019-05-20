// UnknownChunk
pub struct UnknownChunk {
    pub id: String,
}

impl UnknownChunk {
    pub fn new(id: String) -> UnknownChunk {
        UnknownChunk { id }
    }
}