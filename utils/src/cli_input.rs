use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    //x dbg!(args);
    let f: f32 = args.get(1).unwrap().parse::<f32>().unwrap();
    println!("{:?}", f);
}
