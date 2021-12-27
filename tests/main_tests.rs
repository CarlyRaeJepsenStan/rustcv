use image::io::Reader as ImageReader;

#[test]
fn test_load_image() {
    let _image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
}

#[test]
fn test_save_image() {
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    image.save("./tests/output/saved_image_test_file.png").unwrap();
}
