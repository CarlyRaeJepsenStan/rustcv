use image::{DynamicImage, ImageBuffer, Luma, GenericImageView, Pixel};

use crate::utils::to_greyscale;


pub fn calc_first_derivative_image(input: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = input.dimensions();
    let image = to_greyscale(input);

    let mut output = image.clone();

    let mut diff;
    let mut out_pix;
    for i in 0..(height-1) {
        for j in 0..(width-1) {
            diff = (image.get_pixel(j, i).0[0] as i16 - image.get_pixel(j+1, i+1).0[0] as i16).abs() as u8;
            out_pix = output.get_pixel_mut(j, i);
            out_pix.0[0] = diff;
        }
    }

    output
}