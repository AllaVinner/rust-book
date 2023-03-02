use std::ops::{Sub, Add, Mul};
use std::cmp::{PartialEq, Eq};
use num_traits::pow::Pow;

const N: i32 = 5;

#[derive(Debug, Clone, Copy)]
pub struct Z {
    pub x: i32
}

pub trait ModularArithmetic {
    fn resolve(&self) -> Z;
}

impl ModularArithmetic for Z {
    fn resolve(&self) -> Z {
        
        Z{x: (((self.x % N) + N) % N)}
    }
}


impl Add for Z {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x),
        }.resolve()
    }
}

impl Sub for Z {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: (self.x - other.x),
        }.resolve()
    }
}


impl Mul for Z {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: (self.x * other.x),
        }.resolve()
    }
}


impl PartialEq for Z {

    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x) % N == 0
    }
}

impl Eq for Z {}



impl Add<i32> for Z {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self {
            x: (self.x + other),
        }.resolve()
    }
}


impl Sub<i32> for Z {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        Self {
            x: (self.x - other),
        }.resolve()
    }
}


impl Mul<i32> for Z {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: (self.x * other),
        }.resolve()
    }
}

impl Pow<u32> for Z {
    type Output = Self;

    fn pow(self, other: u32) -> Self {
        (0..other).fold(Z{x: 1}, |acc,  _| (acc*self).resolve())
    }
}






