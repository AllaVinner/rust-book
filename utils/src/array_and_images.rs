use std::ops::{Add, Div};
use clap::{Parser, ValueEnum};
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

#[derive(Parser, Debug)]
#[command(author, version, 
    about="THis is a great command", 
    long_about = "Actually This is the greatest that ever existed")]
struct Args {
   #[arg(short, long, default_value_t = String::from("fractal.png"))]
   in_path: String,

   #[arg(short, long, default_value_t = String::from("fractal_out.png"))]
   out_path: String,
}

pub fn main() {
    let args: Args = Args::parse();
    let (height, width) = image_dimensions(&args.in_path).unwrap();
    let image = Array3::from_shape_vec(Dim([height as usize, width as usize,3]), open(args.in_path).unwrap().into_bytes()).unwrap();

    println!("Before");
    let blurred = blur(image.view(), [10,10,1]);
    let bimage:ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(blurred.shape()[0] as u32, blurred.shape()[1] as u32, blurred.into_raw_vec()).unwrap();
    bimage.save(args.out_path).unwrap();
}

