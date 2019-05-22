use std::path::PathBuf;

use clap::{App, Arg};
use std::ffi::{OsString};

pub struct Config {
    pub input_file_path: PathBuf,
    pub output_folder_path: PathBuf,
    pub output_file_stem: Box<OsString>,
}

impl Config {
    pub fn get_output_file_path(&self, ext: &str) -> PathBuf {
        let file = self.output_file_stem.to_os_string();
        self.output_folder_path.join(file).with_extension(ext)
    }

    pub fn from(args: Vec<String>) -> Config {
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
//                .multiple(true)
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
            .get_matches_from(args);

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

        let output_file_stem = match input_file_path.file_stem() {
            Some(p) => p.to_owned(),
            None => panic!("oh no"), // TODO: Result<>
        };

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
            output_file_stem: Box::new(output_file_stem)
        }
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::Config;

    #[test]
    fn from_single_file_name() {
        let config = Config::from(vec![String::from("exe"), String::from("bild.iff")]);

        assert_eq!("bild.iff", config.input_file_path.to_str().unwrap());
        assert_eq!("", config.output_folder_path.to_str().unwrap());
        assert_eq!("bild", config.output_file_stem.to_str().unwrap());
    }

    #[test]
    fn from_single_file_name_with_path() {
        let config = Config::from(vec![String::from("exe"), String::from("bildpath\\bild.iff")]);

        assert_eq!("bildpath\\bild.iff", config.input_file_path.to_str().unwrap());
        assert_eq!("bildpath", config.output_folder_path.to_str().unwrap());
        assert_eq!("bild", config.output_file_stem.to_str().unwrap());
    }

    #[test]
    fn file_name_and_output_path() {
        let config = Config::from(vec![String::from("exe"), String::from("bild.iff"), String::from("-o"), String::from("someotherpath")]);

        assert_eq!("bild.iff", config.input_file_path.to_str().unwrap());
        assert_eq!("someotherpath", config.output_folder_path.to_str().unwrap());
        assert_eq!("bild", config.output_file_stem.to_str().unwrap());
    }

    #[test]
    fn get_output_file_path() {
        let config = Config::from(vec![String::from("exe"), String::from("somefolder\\bild.iff")]);
        let res = config.get_output_file_path("ilbm");
        assert_eq!("somefolder\\bild.ilbm", res.to_str().unwrap());
    }
}

