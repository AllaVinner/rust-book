use std::ops::{Add, Sub};
use std::ops::Mul;



#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x*other,
            y: self.y*other
        }
    }
}



impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: & Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: & Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}


/*
Test script
*/

pub fn main() {
    let mut p = Point{x: 1, y: 3};
    let mut q = Point{x: -1, y: 2};
    let mut r: i32 = 4;
    println!("p + q: {:?}", &p + &q );
    println!("p - q: {:?}", &p - &q );
    println!("r*p: {:?}", &p*r);
    
}