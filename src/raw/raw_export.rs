use crate::config::Config;
use crate::error::ErrorKind;
use crate::io::file_writer;

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
    pub fn export(&self, config: &Config) -> Result<(), ErrorKind> {
        if let Some(cmap) = &self.cmap {
            println!("There is a cmap for export {:?}", cmap);
            let file_path = config.get_output_file_path("cmap");
            file_writer::write_file(&file_path, cmap);
        }
        Ok(())
    }
}