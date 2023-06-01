


fn adder(a: &mut i32, b: & i32) {
    *a = *a+*b;
}

pub fn main() {

    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;

    adder(&mut a, &b);
    dbg!(a);
}


