use image::{io::Reader as ImageReader, GenericImageView};

const TEST_OUTPUT_DIR: &str = "./tests/output/";
const WINDOW_TEST_IMG: &str = "./test_images/window_only.png";

#[test]
fn test_load_image() {
    let _image = ImageReader::open(WINDOW_TEST_IMG).unwrap().decode().unwrap();
}


//these next two tests are both "write to disk" tests,
//and as such, shouldn't need to be run more that once.
// use ' cargo test -- --ignored ' to run these tests
#[test]
#[ignore]
fn test_save_image() {
    let image = ImageReader::open(WINDOW_TEST_IMG).unwrap().decode().unwrap();
    image.save(TEST_OUTPUT_DIR.to_owned() + "saved_image_test_file.png").unwrap();
}

#[test]
#[ignore]
fn test_grayscale() {
    let image = ImageReader::open(WINDOW_TEST_IMG).unwrap().decode().unwrap();
    let image_grey = image.to_luma8();
    image_grey.save(TEST_OUTPUT_DIR.to_owned() + "greyscale_window.png").unwrap();
}

#[test]
fn get_image_dimensions() {
    let image = ImageReader::open(WINDOW_TEST_IMG).unwrap().decode().unwrap();
    //values here found by printing dimensions somwhere else...
    assert_eq!(image.dimensions(), (1516 as u32, 992 as u32));
}