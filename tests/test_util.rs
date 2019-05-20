use gfxconv::error::ErrorKind;

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

pub fn assert_error<T>(expected: ErrorKind, result: Result<T, ErrorKind>) {
    let actual = result.err().unwrap();
    assert_eq!(expected, actual);
}
