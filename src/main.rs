fn main() {
    let result = gfxconv::read_iff_file(std::path::PathBuf::from(
        "D:\\github\\gfxconv\\tests\\test01_320_256_256.iff",
    ));

    match result {
        Ok(res) => println!("Ok: {:?}", res),
        Err(err) => eprintln!("Error: {:?}", err),
    }
    // println!("result: {}", result);
    // println!("result: {}", &(result.unwrap().pop().unwrap()).get_children().pop());// get_children());
}
