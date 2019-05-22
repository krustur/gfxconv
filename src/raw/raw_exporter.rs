use crate::error::ErrorKind;
use crate::raw::raw_export::RawExport;

pub trait RawExporter {
    fn export(&self) -> Result<RawExport, ErrorKind>;
}