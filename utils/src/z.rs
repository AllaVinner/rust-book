use std::ops::{Sub, Add};

const N: i32 = 5;

#[derive(Debug, Clone)]
pub struct Z {
    pub x: i32
}

pub trait Modules {
    fn resolve(&self) -> Z;
}

impl Modules for Z {
    fn resolve(&self) -> Z {
        Z{x: self.x % N}
    }
}


impl Add for Z {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) % N,
        }
    }
}

impl Sub for Z {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: (self.x + 5 - other.x % N) % N,
        }
    }
}








