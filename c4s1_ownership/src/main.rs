fn main() {
    // String type, not string literal
    let mut s = String::from("hello");
    s.push_str(", world");
    println!(" s: {s}");

    // Variables and Data Interacting with Move
    let s1 = s;
    println!("s1: {s1}");
    // error[E0382]: borrow of moved value: `s`
    //println!(" s: {s}");

    // Variables and Data Interacting with Clone
    let s2 = s1.clone();
    println!("s2: {}\ns1: {}", s2, s1);

    // Ownership and Functions
    takes_ownership(s2);
    // println!("{}", s2);
    let x = 5;
    makes_copy(x);
    println!("{x}");

    // Listing 4-5: Returning ownership of parameters
    println!("\n>> Listing 4-5: Returning ownership of parameters");
    return_ownership();
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
}

fn return_ownership() {
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s :String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
