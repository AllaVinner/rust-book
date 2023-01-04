use std::ops::{Add, Sub};
use std::ops::Mul;

use crate::type_of::type_of;


#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Add<Output = T> + Copy> Add<&Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn add(self, other: & Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}



pub fn main() {
    let a = Point {x: 23, y: 11};
    let b = Point {x: -23, y: -11};
    println!("{:?}", a);
    println!("{:?}", type_of(&a));
    println!("{:?}", &a+&   b);
}
