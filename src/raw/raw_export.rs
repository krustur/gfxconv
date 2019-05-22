pub struct RawExport {
    cmap: Option<Vec<u8>>,

}

impl RawExport {
    pub fn new() -> RawExport {
        RawExport{
            cmap: None,
        }
    }

    pub fn add_cmap(&mut self, buffer: Vec<u8> ) {
        self.cmap = Some(buffer);
    }
}