use kru_gfx_conv;

// fn correct_number_of_bitplanes() {
#[test]
fn short_file() {
    let test_path = test_util::get_tests_path().join("short_file.iff");
    assert!(kru_gfx_conv::read_iff_file(test_path).is_err());
}

#[test]
fn not_form_type() {
    let test_path = test_util::get_tests_path().join("not_form.iff");
    assert!(kru_gfx_conv::read_iff_file(test_path).is_err());
}

#[test]
fn unknown_form_type() {
    let test_path = test_util::get_tests_path().join("unknown_form_type.iff");
    assert!(kru_gfx_conv::read_iff_file(test_path).is_err());
}

#[test]
fn zero_size_chunk() {
    let test_path = test_util::get_tests_path().join("zero_size_chunk.iff");
    assert!(kru_gfx_conv::read_iff_file(test_path).is_err());
}

#[test]
fn correct_root_chunk_id() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    assert_eq!(
        "D:\\github\\kru_gfx_conv\\tests\\test01_320_256_256.iff",
        test_path.to_str().unwrap()
    );
    assert_eq!(
        "FORM",
        kru_gfx_conv::read_iff_file(test_path)
            .unwrap()
            .pop()
            .unwrap()
            .get_id()
    );
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
