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
mod parallelization;
mod benchmark;

fn main() {
    benchmark::main();
}
