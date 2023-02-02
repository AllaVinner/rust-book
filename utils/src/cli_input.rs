use clap::{Parser, ValueEnum};
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    basic_arg: String,

   #[arg(short, long, default_value_t = -0.5)]
   basic_option: f32,

   #[arg(short, long, default_value_t = String::from("Default Input"))]
   string: String,

   #[arg(value_enum, short='a', long, default_value_t = Category::A)]
   category: Category,
}

#[derive(ValueEnum, Clone, Debug)]
enum Category {
    A,
    B,
}

pub fn main() {
    let args: Args = Args::parse();
    dbg!(args);
}
