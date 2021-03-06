use image::{io::Reader as ImageReader, GenericImageView};

mod first_derivative;
mod utils;

//alrighty, this is bad practice, but it works.
//Each test should be in the same file as the code it is for
//but I already did this. Soo....
#[cfg(test)]
mod tests;


//TODO:
//Resize to something smaller and square, like 512x512
//Import nalgebra and make the vec into a square matrix
//Figure out an algorithm to do something like conv.mp4 - inner/outer product?


fn main() {
   //Import and image, make it grey and then turn into vec  
   let image = ImageReader::open("../test_images/window_only.png").unwrap().decode().unwrap();
    let gray_pic = utils::to_greyscale(&image);
    println!("{:?}", gray_pic[(0,0)]);
    let gray_vec = gray_pic.into_vec();
    println!("{}, {}", gray_vec[0], gray_vec.len());
}

fn create_first_derivative_nick_1() {
    let image = ImageReader::open("./test_images/window_only.png").unwrap().decode().unwrap();
    let first_div = first_derivative::calc_first_derivative_image_horizontal(&image);
    first_div.save("./output/first_div_horizontal.png").unwrap();
}
