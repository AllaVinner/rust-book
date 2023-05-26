
use std::fs::File;

fn basic_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("I caught the ERROR >> Problem opening the file: {:?}", error),
    };
}

pub fn main() {
    println!("Basic Error Handling.");
    basic_error_handling();

}


