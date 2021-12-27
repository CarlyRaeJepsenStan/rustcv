use image::io::Reader as ImageReader;

const TEST_OUTPUT_DIR: &str = "./tests/output/";

#[test]
fn test_load_image() {
    let _image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
}

#[test]
fn test_save_image() {
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    image.save(TEST_OUTPUT_DIR.to_owned() + "saved_image_test_file.png").unwrap();
}


#[test]
fn test_grayscale() {
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    let image_grey = image.to_luma8();
    image_grey.save(TEST_OUTPUT_DIR.to_owned() + "greyscale_window.png").unwrap();
}