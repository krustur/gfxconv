use clap::{App, Arg};
use std::path::PathBuf;

pub struct Config{
    pub input_file_path: PathBuf,
    pub output_folder_path: PathBuf,
//    pub output_file_name: String,
}

impl Config {
    pub fn get_config() -> Config {
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
        let input_file_path = std::path::PathBuf::from(input);
        println!("input_file_path: {:?}", input_file_path);

        let input_parent_path = match input_file_path.parent() {
            Some(p) => p,
            None => panic!("oh no"), // TODO: Result<>
        };
        let output_path = matches.value_of("output-path");
        let output_folder_path = match output_path {
            Some(o) => std::path::PathBuf::from(o),
            None => std::path::PathBuf::from(input_parent_path),
        };
        println!("output_folder_path: {:?}", output_folder_path);
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

        Config {
            input_file_path,
            output_folder_path,

        }
    }
}