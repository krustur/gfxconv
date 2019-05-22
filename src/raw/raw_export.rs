use crate::config::Config;
use crate::error::ErrorKind;

pub struct RawExport {
    cmap: Option<Vec<u8>>,

}

impl RawExport {
    pub fn new() -> RawExport {
        RawExport {
            cmap: None,
        }
    }

    pub fn add_cmap(&mut self, buffer: Vec<u8>) {
        self.cmap = Some(buffer);
    }

    #[allow(irrefutable_let_patterns)]
    pub fn export(self, _config: &Config) -> Result<(), ErrorKind> {
        if let cmap = Some(self.cmap) {
            println!("There is a cmap for export {:?}", cmap);

        }
        Ok(())
    }
}