fn main() {
    let _result = kru_gfx_conv::read_iff_file(std::path::PathBuf::from(
        "D:\\github\\kru_gfx_conv\\tests\\test01_320_256_256.iff",
    ));

    // println!("result: {}", result);
    // println!("result: {}", &(result.unwrap().pop().unwrap()).get_children().pop());// get_children());
}
