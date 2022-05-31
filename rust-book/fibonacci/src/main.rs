use std::io;

fn main() {
    println!("Please enter a number");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let n: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not parse input to integer");
            2
        }
    };

    let fibn = get_fib(n);
    println!("Fib number {} is {}", n, fibn);
}


fn get_fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    get_fib(n-1) + get_fib(n-2)
}

