use image::{DynamicImage, ImageBuffer, Luma};

use crate::utils::to_greyscale;


fn calc_first_derivative_image(input: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let image = to_greyscale(input);

    image
}