use image::{GenericImage, GenericImageView, ImageBuffer, open, image_dimensions};
use ndarray::{Array3, Dim};

pub fn main() {
    let (height, width) = image_dimensions("fractal.png").unwrap();
    let image = Array3::from_shape_vec(Dim([height as usize, width as usize,3]), open("fractal.png").unwrap().into_bytes()).unwrap();
    
     
    println!("{:?}", image.dim());
}









