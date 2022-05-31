use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let sectret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input you guess ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("Your guess was {}", guess);

}
