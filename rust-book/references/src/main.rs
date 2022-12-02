fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let a = 123;
    // copy
    let b = a;

    let s1 = String::from("Hello");
    let ref_s1 = &s1;
    println!("Type of ref_s1 is: {}", type_of(&ref_s1));

    let mut s2 = String::from("Hello 2");
    let ref_s2 = &mut s2;
    println!("Type of ref_s2 is: {}", type_of(&ref_s2));

    
}
