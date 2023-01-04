// Return type of variable
fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}

/*
TEST FUNCTION
*/
use std::fmt::Display;
struct MyType;

fn fun(a: &impl Display){
    println!("Type of a in fun: {:?}", type_of(&a));    
}

pub fn main() {
    let i: i32 = 123;
    let s: String = "AAA".to_string();
    let mine = MyType;
    println!("Type of i: {:?}", type_of(&i));
    println!("Type of s: {:?}", type_of(&s));
    println!("Type of mine: {:?}", type_of(&mine));
    fun(&(4 as i32));
}