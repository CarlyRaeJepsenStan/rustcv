use image::{DynamicImage, ImageBuffer, Luma};
use nalgebra::DMatrix;


pub fn to_greyscale(input: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let greyscale_input = input.to_luma8();

    greyscale_input
}

pub fn pad(mt: DMatrix<f32>) -> DMatrix<f32> {
    let r = mt.nrows();
    let c = mt.ncols();
    let padded = mt.insert_row(r,0.0).insert_row(0,0.0).insert_columns(c,1,0.0).insert_columns(0,1,0.0);
    return padded;
}
