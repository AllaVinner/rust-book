
use std::ops::Add;

const DIM: usize = 2;

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
        let mut new = Point {x: [0; DIM]};
        for i in 0..DIM {
            new.x[i] = self.x[i] + other.x[i]; 
        }
        new
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        let mut new = Point {x: [0; DIM]};
        for i in 0..DIM {
            new.x[i] = self.x[i] + other.x[i]; 
        }
        new
    }
}


impl Add for &mut Point {
    type Output = Point;

    fn add(self, other: &mut Point) -> Point {
        let mut new = Point {x: [0; DIM]};
        for i in 0..DIM {
            new.x[i] = self.x[i] + other.x[i]; 
        }
        new
    }
}




fn infun(a: &mut Point, b: &mut Point) {
    *a = *a + *b;
    println!("jjjj{:?}", a);
    
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
    infun(&mut a, &mut b);
    println!("a+b: {:?}",a);    
}










