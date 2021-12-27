use image::{io::Reader as ImageReader, GenericImageView};

mod first_derivative;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    println!("{:?}", image.dimensions());
}
