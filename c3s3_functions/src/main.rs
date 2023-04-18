fn main() {
    println!("Hello, world!");

    another_function();
    second_another_function(10);
    print_labeled_measurement(5, 'h');
    expressions();
}

fn another_function(){
    println!("Another function.");
}

fn second_another_function(x:i32){
    println!("The value od x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_lebel: char) {
    println!("The measurement is: {}{}", value, unit_lebel);
}

fn expressions() {
    let y = { let x = 3; x + 1 };
    println!("the value of y is: {}", y);
}