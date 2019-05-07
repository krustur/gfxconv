use kru_gfx_conv;

#[test]
// fn correct_number_of_bitplanes() {
fn correct_width() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    // assert_eq!(
    //     "D:\\github\\kru_gfx_conv\\tests\\test01_320_256_256.iff",
    //     test_path.to_str().unwrap()
    // );
    assert_eq!(320, kru_gfx_conv::read_iff_file(test_path).unwrap().width);
}

mod test_util {
    pub fn get_tests_path() -> std::path::PathBuf {
        let exe_path = std::env::current_exe().unwrap();
        let test_path = exe_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("tests");
        // let test_path_str = test_path.to_str().unwrap();
        return test_path;
    }
}
