

fn basic_use() {
    println!("Basic usage");
    let a = vec![1,5,4];
    // Map takes a closure as input
    dbg!(a.iter().map(|v| v+10));
}

fn main() {
    basic_use();
}
