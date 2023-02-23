use image::{GenericImage, GenericImageView, ImageBuffer, open, image_dimensions};
use ndarray::{Array3};

pub fn main() {
    let image = open("fractal.png").unwrap();
    let (height, width) = image_dimensions("fractal.png").unwrap(); 
    println!("{:?}", height);
}









