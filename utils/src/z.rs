

const N: u32 = 5;

pub struct Z {
    pub x: i32
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








