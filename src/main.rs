use image::{io::Reader as ImageReader, GenericImageView};

mod first_derivative;
mod utils;

//alrighty, this is bad practice, but it works.
//Each test should be in the same file as the code it is for
//but I already did this. Soo....
#[cfg(test)]
mod tests;

fn main() {
    
}

fn create_first_derivative_nick_1() {
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    let first_div = first_derivative::calc_first_derivative_image_horizontal(&image);
    first_div.save("./output/first_div_horizontal.png").unwrap();
}
