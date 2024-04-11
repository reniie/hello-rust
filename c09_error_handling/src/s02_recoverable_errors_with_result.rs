use std::fs::File;
use std::io::ErrorKind;

pub fn body() {
    println!("\nListing 9-3: Opening a file");
    let greeting_file_result = File::open("hello.txt");
    
    println!("\nListing 9-4: Using a match expression to handle the Result variants that might be returned");
    let greeting_file_result = File::open("/Users/renie/Project/hello-rust/c09_error_handling/src/s01_unrecoverable_errors_with_panic.rs");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("greeting_file: {:?}", greeting_file);
}

pub fn matching_on_different_errors() {
    println!("\nListing 9-5: Handling different kinds of errors in different ways");
    let path = "file-demo.txt";
    let greeting_file_result = File::open(path);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
    println!("greeting_file: {:?}", greeting_file);
}

pub fn alternatives_to_using_match_with_result_t_e(){
    println!("\nListing 9-5: Handling different kinds of errors in different ways");
    let path = "file-demo.txt";
    let greeting_file = File::open("path").unwrap_or_else(|error| {
        if (error.kind() == ErrorKind::NotFound) {
            File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem creating the file: {:?}", error);
        }
    });
    println!("greeting_file: {:?}", greeting_file);
}

pub fn shortcuts_for_panic_on_error_unwrap_and_expect(){
    // let greeting_file = File::open("file-demo.txt").unwrap();
    let greeting_file = File::open("file-demo.txt").expect("file should be included in this project");
    println!("greeting_file: {:?}", greeting_file);
}