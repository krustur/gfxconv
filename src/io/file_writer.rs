pub fn write_file(file_path: &PathBuf, buffer: &[u8]) -> Result<(), Error> {
    println!("file_path {:?}", file_path);

//    let mut file = match File::open(file_path) {
//        Ok(file) => file,
//        Err(error) => return Err(error),
//    };
//    let mut buffer = Vec::new();
//    match file.read_to_end(&mut buffer) {
//        Ok(_) => (),
//        Err(error) => return Err(error),
//    };

    Ok()
}