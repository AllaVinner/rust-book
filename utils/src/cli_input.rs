use clap::{Parser, ValueEnum};
use clap;
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, 
    about="THis is a great command", 
    long_about = "Actually This is the greatest that ever existed")]
struct Args {
    #[arg()]
    basic_arg: String,

   #[arg(short, long, default_value_t = -0.5)]
   basic_option: f32,
   
   #[arg(short, long, default_value = None)]
   optional_option: Option<f32>,

   #[arg(short, long, default_value_t = String::from("Default Input"))]
   string: String,

   #[arg(value_enum, short='a', long, default_value_t = Category::A, 
   long_help="This command is a bit more complex than the others")]
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
