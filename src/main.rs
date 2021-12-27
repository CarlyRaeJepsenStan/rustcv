use image::{io::Reader as ImageReader, GenericImageView};

mod first_derivative;

fn main() {
    
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    println!("{:?}", image.dimensions());
}
