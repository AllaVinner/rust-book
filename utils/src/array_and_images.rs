use std::ops::{Add, Div};

use image::{GenericImage, GenericImageView, ImageBuffer, open, image_dimensions, Rgb};
use ndarray::{Array3, Dim, ArrayView3, IntoDimension, Array};
use num_traits::{FromPrimitive, Zero, zero};


fn blur<'a, E>(view: ArrayView3<'a, u8>, window_dim: E) -> Array3<u8> 
where
    E: IntoDimension<Dim = Dim<[usize; 3]>> + Copy
{
    let out_dim = view.raw_dim() - Dim(window_dim) + Dim([1,1,1]);
    Array::from_iter(view.windows(window_dim).into_iter().map(|w| (w.iter().map(|v| *v as u32).sum::<u32>() / 100) as u8))
        .into_shape(out_dim)
        .unwrap()
}


pub fn main() {
    let (height, width) = image_dimensions("fractal.png").unwrap();
    let image = Array3::from_shape_vec(Dim([height as usize, width as usize,3]), open("fractal.png").unwrap().into_bytes()).unwrap();

    let blurred = blur(image.view(), [10,10,1]);
    let bimage:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, blurred.into_raw_vec()).unwrap();
    bimage.save("fractal_blurr.png").unwrap();

}









