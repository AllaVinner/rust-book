
use std::fs::File;

fn basic_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("I caught the ERROR >> Problem opening the file: {:?}", error),
    };
}

fn handle_different_errors() {
        let greeting_file_result = File::open("hello.txt");
    
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
}


pub fn main() {
    println!("Basic Error Handling.");
    basic_error_handling();

    println!("Handle Different Errors");
    handle_different_errors();

}


