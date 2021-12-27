use image::{io::Reader as ImageReader, GenericImageView};

mod first_derivative;
mod utils;

//alrighty, this is bad practice, but it works.
//Each test should be in the same file as the code it is for
//but I already did this. Soo....
#[cfg(test)]
mod tests;

fn main() {
    
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    println!("{:?}", image.dimensions());
}
