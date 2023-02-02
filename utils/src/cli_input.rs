use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    basic_option: String,

   #[arg(short, long, default_value_t = -0.5)]
   float: f32,

   #[arg(short, long, default_value_t = 0.3)]
   imag: f32,

   #[arg(short, long, default_value_t = 0.3)]
   optional: Option<i32>,

   #[arg(value_enum, short='a', long, default_value_t = Fractal::Julia)]
   fractal: Fractal,
}

#[derive(ValueEnum, Clone, Debug)]
enum Fractal {
    Julia,
    Mandelbrot
}

pub fn main() {
    let args: Args = Args::parse();
    dbg!(args);
}
