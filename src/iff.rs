use crate::error::ErrorKind;
use crate::iff::chunks::form_ilbm_chunk::FormIlbmChunk;
use crate::raw::raw_export::RawExport;
use crate::raw::raw_exporter::RawExporter;

pub mod chunks;
pub mod raw;

#[derive(Debug)]
pub struct IffFile {
    pub ilbm: FormIlbmChunk,
}

impl IffFile {
    pub fn from_iff_buffer(buffer: &[u8]) -> Result<IffFile, ErrorKind> {
        if buffer.len() < 12 {
            return Err(ErrorKind::FileTooShort);
        }

        let iff_file = IffFile {
            ilbm: FormIlbmChunk::parse_form_ilbm_buffer(buffer)?,
        };

        Ok(iff_file)
    }
}

impl RawExporter for IffFile{
    fn export(&self) ->  Result<RawExport, ErrorKind> {
        let mut raw_export = RawExport::new();

//        let cmap = match &self.ilbm.cmap{
//            None => {None},
//            Some(c) => {Some(c)},
//        };

        match &self.ilbm.cmap{
            None => {},
            Some(c) => {
                let mut cmap = vec![0; c.rgb.len()*3];
                for i in 0..c.rgb.len() {
                    cmap[i*3+0] = c.rgb[i].r;
                    cmap[i*3+1] = c.rgb[i].g;
                    cmap[i*3+2] = c.rgb[i].b;
                }

//                println!("c.rgb {:?}", c.rgb);
//                println!("cmap {:?}", cmap);
                raw_export.add_cmap(cmap);
            },
        }

        Ok(raw_export)
    }
}