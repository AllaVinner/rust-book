use std::time::Instant;

pub fn main() {
    let num_iterations = 10_000;
    let now = Instant::now();
    for _ in 0..num_iterations {
         let v: Vec<u32> = (0..num_iterations).into_iter().map(|x| x % 7).collect();
    }
    let elapsed = now.elapsed(); 
    let rate = elapsed / num_iterations;
    println!("Total time was {:?} for {:?} iterations with an average time of {:?}", elapsed, num_iterations, rate);
}
