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



fn main() {
    for f in init_fibonacci() {
        println!("Next fibonacci is {:?}", f);
        if f > 10000 {
            break;
        }
    }
    
    let fib_sum: u32 = init_fibonacci().map(|f| f/2).take_while(|f| *f < 1000).sum();

    println!("Sum of half fib is {:?}", fib_sum);
}
