
use std::ops::Add;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: [i32; 2]
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: [self.x[0] + other.x[0], self.x[1] + other.x[1]]}
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {x: [self.x[0] + other.x[0], self.x[1] + other.x[1]]}
    }
}

impl Add for &mut Point {
    type Output = Point;

    fn add(self, other: &mut Point) -> &mut Point {
        Point {x: [self.x[0] + other.x[0], self.x[1] + other.x[1]]}
    }
}


fn infun(a: &mut Point, b: &mut Point) {
    *a = a + b;
    
}




pub fn main() {
    let mut a = Point {x: [1, 2]};
    let mut b = Point {x: [3, 4]};
    let mut c = Point {x: [5, 6]};
    

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("&a+&b: {:?}", &a+&b);
    let c = &a+&b;
    
    println!("a+b: {:?}",type_of(&c));
    println!("a+b: {:?}",a);
    infun(&mut a, &b, &c);
    println!("a+b: {:?}",a);    
}










