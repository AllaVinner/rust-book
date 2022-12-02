fn main() {
    let mut s = String::from("hello");  // s comes into scope
    
                                  // ... and so is no longer valid here
    {
        //  let mut s = String::from(" AAA ");
        println!("Inner scope {}", s);
        s.push_str(", worrld");
        println!("Inner scope after {}", s);
    }
    
    println!("outer scope {}", s);
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
