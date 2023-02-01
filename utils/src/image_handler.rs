use image::io::Reader as ImageReader;
use image;
use num_complex;
use core::panic;
use std::env;
use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t = -0.5)]
   real: f32,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 0.3)]
   imag: f32,

   #[arg(value_enum, short, long, default_value_t = Fractal::Julia)]
   fractal: Fractal,
}

#[derive(ValueEnum, Clone, Debug)]
enum Fractal {
    Julia,
    Mandelbrot
}

pub fn main() {
    let args: Args = Args::parse();
    fractal(args.real, args.imag, args.fractal);
}

fn fractal(constant_real: f32, const_imag: f32, fractal: Fractal) {
    match fractal {
        Fractal::Julia => julia_fractal(constant_real, constant_real),
        Fractal::Mandelbrot => mandelbrot_fractal(constant_real, constant_real),
    }
}

fn julia_generator(c_re: f32, c_im: f32) -> impl Fn() {
    

    let mut i = 0;
    while i < 255/20 && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    
}

fn julia_fractal(c_re: f32, c_im: f32) {
    let c = num_complex::Complex::new(c_re, c_im);
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255/20 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i*20 as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal2.png").unwrap();
}

fn mandelbrot_fractal(cx: f32, cy: f32) {
    let z0 = num_complex::Complex::new(cx, cy);
    let imgx = 2300;
    let imgy = 2300;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let g = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([0, g, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 2.;
            let cy = x as f32 * scaley - 1.5;

            let mut c = num_complex::Complex::new(cx, cy);
            let mut z = z0;
            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([i as u8, data[1], data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal2.png").unwrap();
}



