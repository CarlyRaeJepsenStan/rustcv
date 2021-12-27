use image::io::Reader as ImageReader;

#[test]
fn test_load_image() {
    let _image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
}