fn main() {
    println!(">> if expressions:");
    if_expressions();

    println!("\n>> multiple conditions:");
    multiple_conditions();

    println!("\n>> using if in a let statement");
    if_in_let();

    println!("\n>> expriment loops");
    loops();
}

// if Expressions
fn if_expressions(){
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// Handling Multiple Conditions with else if
fn multiple_conditions(){
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Using if in a let Statement
fn if_in_let(){
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "6" };
    println!("The value of number is: {number}");
}

// expriment
fn loops() {
    // loop {
    //     println!("again!");
    // }
    println!(">>>> return values:");
    return_values();
    println!(">>>> loop label:");
    loop_label();
    println!(">>>> while loop I");
    while_loop_1();
    println!(">>>> while loop II");
    while_loop_2();
    println!(">>>> countdown");
    countdown();
}

// return values from loops
fn return_values(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// loop label
fn loop_label(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

// while loop
fn while_loop_1(){
    let mut number = 5;
    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("LIFTOFF!")
}

fn while_loop_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("[while] the value is: {}", a[index]);
        index += 1;
    }

    for elem in a {
        println!("[for] the value is : {elem}")
    }
}

// number countdown
fn countdown(){
    for num in (1..5).rev() {
        println!("{num} ");
    }
    println!("LIFTOFF");
}