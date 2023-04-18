use std::io;

fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f :bool = false;

    // character
    let c = 'z';
    let z :char = 'a';

    // tuple
    let tup :(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The Value of y is {y}");

    // array
    let months = ["January", "Fedruary", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"];
    let a :[i32; 5] = [1, 2, 3, 4, 5];
    let b = [0; 10];

    // array access
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line!");

    let index :usize = index.trim().parse().expect("Index entered eas not an number.");
    let element = a[index];
    println!("The Value of the element at index {index} is {element}");
}
