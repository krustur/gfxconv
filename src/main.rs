//#[allow(unused_imports)]
//mod raw;

use std::process;

use crate::config::Config;
use crate::iff::IffFile;
use crate::io::file_reader;
use crate::raw::raw_exporter::RawExporter;

pub mod error;
pub mod common;
pub mod config;
pub mod io;
pub mod iff;
pub mod raw;

//use crate::raw;
//use crate::buffer_reader;

fn main() {
    let config = Config::get_config();

    let buffer = match file_reader::read_file(&config.input_file_path) {
        Ok(b) => { b }
        Err(err) => {
            eprintln!("Error while reading input file: {:?}", err);
            process::exit(1);
        }
    };

    let iff_file = IffFile::from_iff_buffer(&buffer);

    let iff_file = match iff_file {
        Ok(res) => (res),
        Err(err) => {
            eprintln!("Error while parsing iff buffer: {:?}", err);
            process::exit(1);
        }
    };

    let raw_export = match iff_file.export() {
        Ok(e) => { e }
        Err(err) => {
            eprintln!("Error while exporting iff: {:?}", err);
            process::exit(1);
        }
    };

    match raw_export.export(&config) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error while exporting iff: {:?}", err);
            process::exit(1);
        }
    }


//    eprintln!("raw_export: {:?}", raw_export);

//    raw::write_file();


    // println!("result: {}", result);
    // println!("result: {}", &(result.unwrap().pop().unwrap()).get_children().pop());// get_children());
}
