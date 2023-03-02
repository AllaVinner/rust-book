use crate::modular_arithmetic::{ModularArithmetic, Z};
use num_traits::pow::Pow;

mod type_of;
mod point_struct;
mod generic_point;
mod image_handler;
mod cli_input;
mod function_defaults;
mod arrays;
mod array_and_images;
mod modular_arithmetic;

fn main() {
    let a = Z{x: 34};

    println!("{:?}", &a);
    println!("{:?}", a.resolve());
    println!("{:?}", a + a);
    println!("{:?}", a-a);
    
    println!("{:?}", a*a);
    println!("{:?}", a == a);
    println!("{:?}", a == a*a);
    println!("{:?}", a+1);
    println!("{:?}", a.pow(120));
}
