use crate::raw::raw_export::RawExport;
use crate::error::ErrorKind;

pub trait RawExporter{
    fn export(&self) -> Result<RawExport, ErrorKind>;
}