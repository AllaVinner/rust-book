use std::str::Chars;

fn consuming_adaptor() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn iterator_adaptor() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.map(|x| x+1).sum();

}

struct Fibonacci {
    current: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = self.current + self.next;
        Some(self.next)
    }
}

fn init_fibonacci() -> Fibonacci {
    Fibonacci {current:0, next:1}
}

struct CargoValue<'a> {
    iter: Chars<'a>
}

impl<'a> Iterator for CargoValue<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next();
        self.iter.next();
        self.iter.next();
        self.iter.next()
    }
}

trait CargoIterator {
    fn cargo_value(&self) -> CargoValue<'_>;
}

impl CargoIterator for &str {
    fn cargo_value(&self) -> CargoValue<'_> {
        CargoValue {iter: self.chars()}
    }
}


fn main() {
    for f in init_fibonacci() {
        println!("Next fibonacci is {:?}", f);
        if f > 10000 {
            break;
        }
    }
    
    let fib_sum: u32 = init_fibonacci().map(|f| f/2).take_while(|f| *f < 1000).sum();

    println!("Sum of half fib is {:?}", fib_sum);

    let s: &str = "som1thi2ads3dfewrtwetwtert";

    for c in s.cargo_value() {
        println!("Next value from s is {:?}", c);
    }
    println!("{:?}", s.cargo_value().count())

}
