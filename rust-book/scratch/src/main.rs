fn main() {
    let x = 5;
    {
        println!("This is x before {}", x);
        let x = x * 2;
        println!("This is x after {}", x);
    }
    
    println!("This is x outside {}", x);
}
