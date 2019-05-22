use crate::config::Config;
use crate::io::file_writer;
use std::io::Error;

pub struct RawExport {
    cmap: Option<Vec<u8>>,
    ilbm: Option<Vec<u8>>,
}

impl RawExport {
    pub fn new() -> RawExport {
        RawExport {
            cmap: None,
            ilbm: None,
        }
    }

    pub fn add_cmap(&mut self, buffer: Vec<u8>) {
        self.cmap = Some(buffer);
    }
    pub fn add_ilbm(&mut self, buffer: Vec<u8>) {
        self.ilbm = Some(buffer);
    }

    #[allow(irrefutable_let_patterns)]
    pub fn export(&self, config: &Config) -> Result<(), Error> {
        if let Some(cmap) = &self.cmap {
            let file_path = config.get_output_file_path("cmap");
            file_writer::write_file(&file_path, cmap)?;
        }
        if let Some(ilbm) = &self.ilbm {
            let file_path = config.get_output_file_path("ilbm");
            file_writer::write_file(&file_path, ilbm)?;
        }
        Ok(())
    }
}