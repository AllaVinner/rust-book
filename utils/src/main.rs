use crate::z::{Modules};

mod type_of;
mod point_struct;
mod generic_point;
mod image_handler;
mod cli_input;
mod function_defaults;
mod arrays;
mod array_and_images;
mod z;

fn main() {
    let mut a = z::Z{x: 34};

    println!("{:?}", &a);
    println!("{:?}", a.resolve());
    println!("{:?}", a);
    println!("{:?}", a);
}
