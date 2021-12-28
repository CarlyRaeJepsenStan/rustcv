use image::{DynamicImage, ImageBuffer, Luma};


pub fn to_greyscale(input: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let greyscale_input = input.to_luma8();

    greyscale_input
}