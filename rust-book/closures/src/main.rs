

fn basic_use() {
    println!("Basic usage");
    let a = vec![1,5,4];
    // Map takes a closure as input
    dbg!(a.iter().map(|v| v+10));
}

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

fn main() {
    basic_use();
}
