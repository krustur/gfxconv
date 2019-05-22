use crate::raw::raw_export::RawExport;

pub trait RawExporter{
    fn export(&self) ->  RawExport;
}