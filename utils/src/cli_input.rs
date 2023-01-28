use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
