//#[allow(unused_imports)]
//mod raw;

use clap::{App, Arg};
use crate::io::file_reader;
use crate::iff::IffFile;
use std::process;
use crate::raw::raw_exporter::RawExporter;

pub mod error;
pub mod common;
pub mod io;
pub mod iff;
pub mod raw;

//use crate::raw;
//use crate::buffer_reader;

fn main() {
    let matches = App::new("GfxConv")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Krister Jansson")
        .about("Converts gfx-files (only IFF-ILBM for now) to custom Raw formats")
//        .arg(Arg::with_name("config")
//            .short("c")
//            .long("config")
//            .value_name("FILE")
//            .help("Sets a custom config file")
//            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Specifies the input file(s) to convert")
            .required(true)
            .multiple(true)
            .index(1))
        .arg(Arg::with_name("output-path")
            .short("o")
            .long("output-path")
            .takes_value(true)
            .help("Specifies the path to where output files will be written"))

//        .arg(Arg::with_name("v")
//            .short("v")
//            .multiple(true)
//            .help("Sets the level of verbosity"))
//        .subcommand(SubCommand::with_name("test")
//            .about("controls testing features")
//            .version("1.3")
//            .author("Someone E. <someone_else@other.com>")
//            .arg(Arg::with_name("debug")
//                .short("d")
//                .help("print debug information verbosely")))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();

    println!("INPUT: {}", input);
    let output_path = matches.value_of("output-path");
    match output_path {
        Some(_o) => println!("output_path: {}", output_path.unwrap()),
        None => (),
    }

//
//    // Vary the output based on how many times the user used the "verbose" flag
//    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }
//
//    // You can handle information about subcommands by requesting their matches by name
//    // (as below), requesting just the name used, or both at the same time
//    if let Some(matches) = matches.subcommand_matches("test") {
//        if matches.is_present("debug") {
//            println!("Printing debug info...");
//        } else {
//            println!("Printing normally...");
//        }
//    }

    // more program logic goes here...

    let input_path = std::path::PathBuf::from(input);
    let buffer = match file_reader::read_file(input_path){
        Ok(b) => {b},
        Err(err) => {
            eprintln!("Error while reading input file: {:?}", err);
            process::exit(1);
        },
    };

    let iff_file = IffFile::from_iff_buffer(&buffer);

    let iff_file = match iff_file {
        Ok(res) => (res),
        Err(err) => {
            eprintln!("Error while parsing iff buffer: {:?}", err);
            process::exit(1);
        }
    };

    let raw_export = match iff_file.export(){
        Ok(e) => {e},
        Err(err) => {
            eprintln!("Error while exporting iff: {:?}", err);
            process::exit(1);
        }
    };
//    eprintln!("raw_export: {:?}", raw_export);

//    raw::write_file();


    // println!("result: {}", result);
    // println!("result: {}", &(result.unwrap().pop().unwrap()).get_children().pop());// get_children());
}
