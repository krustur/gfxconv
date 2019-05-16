use gfxconv;
use gfxconv::ErrorKind;

// fn correct_number_of_bitplanes() {
#[test]
fn short_file() {
    let test_path = test_util::get_tests_path().join("short_file.iff");
    let res = gfxconv::read_iff_file(test_path);
    test_util::assert_error(ErrorKind::FileTooShort, res);
}

#[test]
fn not_form_type() {
    let test_path = test_util::get_tests_path().join("not_form.iff");
    let res = gfxconv::read_iff_file(test_path);
    test_util::assert_error(ErrorKind::UnknownChunk(String::from("FORN")), res);
}

#[test]
fn unknown_form_type() {
    let test_path = test_util::get_tests_path().join("unknown_form_type.iff");
    let res = gfxconv::read_iff_file(test_path);
    test_util::assert_error(ErrorKind::UnknownFormType, res);
}

#[test]
fn zero_size_chunk() {
    let test_path = test_util::get_tests_path().join("zero_size_chunk.iff");
    let res = gfxconv::read_iff_file(test_path);
    test_util::assert_error(ErrorKind::ZeroSizeChunk, res);
}

#[test]
fn correct_root_chunk_id() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    let ilbm = gfxconv::read_iff_file(test_path).unwrap().ilbm;
    assert_eq!("FORM", ilbm.id);
}

#[test]
fn correct_bmhd() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    let bmhd = gfxconv::read_iff_file(test_path)
        .unwrap()
        .ilbm
        .bmhd
        .unwrap();

    assert_eq!(320, bmhd.width, "bmhd width");
    assert_eq!(256, bmhd.height, "bmhd heigth");
    assert_eq!(0, bmhd.x, "bmhd x");
    assert_eq!(0, bmhd.y, "bmhd y");
    assert_eq!(8, bmhd.number_of_planes, "bmhd number_of_planes");
    assert_eq!(0, bmhd.masking, "bmhd masking");
    assert_eq!(1, bmhd.compression, "bmhd compression");
    assert_eq!(
        0, bmhd.transparent_color_number,
        "bmhd transparent_color_number"
    );
    assert_eq!(44, bmhd.x_aspect, "bmhd x_aspect");
    assert_eq!(44, bmhd.y_aspect, "bmhd y_aspect");
    assert_eq!(320, bmhd.page_width, "bmhd page_width");
    assert_eq!(256, bmhd.page_height, "bmhd page_height");
}

#[test]
fn correct_cmap() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    let cmap = gfxconv::read_iff_file(test_path)
        .unwrap()
        .ilbm
        .cmap
        .unwrap();

    assert_eq!(256, cmap.rgb.len(), "cmap.rgb.len");
    assert_eq!(0x00, cmap.rgb[0].r, "cmap.rgb0.r");
    assert_eq!(0x00, cmap.rgb[0].g, "cmap.rgb0.g");
    assert_eq!(0x00, cmap.rgb[0].b, "cmap.rgb0.b");
    assert_eq!(0xa0, cmap.rgb[1].r, "cmap.rgb1.r");
    assert_eq!(0xa0, cmap.rgb[1].g, "cmap.rgb1.g");
    assert_eq!(0xa0, cmap.rgb[1].b, "cmap.rgb1.b");
    assert_eq!(0xff, cmap.rgb[3].r, "cmap.rgb3.r");
    assert_eq!(0xfb, cmap.rgb[3].g, "cmap.rgb3.g");
    assert_eq!(0x00, cmap.rgb[3].b, "cmap.rgb3.b");

    assert_eq!(0xed, cmap.rgb[222].r, "cmap.rgb222.r");
    assert_eq!(0xcc, cmap.rgb[222].g, "cmap.rgb222.g");
    assert_eq!(0xbe, cmap.rgb[222].b, "cmap.rgb222.b");

    assert_eq!(0xff, cmap.rgb[255].r, "cmap.rgb255.r");
    assert_eq!(0xff, cmap.rgb[255].g, "cmap.rgb255.g");
    assert_eq!(0xff, cmap.rgb[255].b, "cmap.rgb255.b");
}

#[test]
fn correct_body() {
    let test_path = test_util::get_tests_path().join("test01_320_256_256.iff");

    let body = gfxconv::read_iff_file(test_path)
        .unwrap()
        .ilbm
        .body
        .unwrap();

    assert_eq!(0x0000CB55, body.raw_buffer.len(), "body.raw_buffer.len");
    assert_eq!(256, body.pixels.len(), "body.pixels.len");
}

mod test_util {
    use gfxconv::ErrorKind;

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
}
