use std::cmp::Ordering;

struct A {
    a: i32
}


struct B {
    b: i32
}

// Equivilance Relations between different classes.

impl PartialEq<B> for A {
    fn eq(&self, other: &B) -> bool {
        self.a == other.b
    }
}

impl PartialEq<A> for B {
    fn eq(&self, other: &A) -> bool {
 
        other == self
    }
}


impl PartialOrd<B> for A {
    fn partial_cmp(&self, other: &B) -> Option<Ordering> {
        Some(self.a.cmp(&other.b))
    }
}


pub fn main() {
    let a = A{a: 1};
    let b = B{b: 2};
    println!("a == b {:?}", a == b);
    println!("a == b {:?}", b == a);
    println!("a < b {:?}", a < b);
    println!("a < b {:?}", a <= b);
}








