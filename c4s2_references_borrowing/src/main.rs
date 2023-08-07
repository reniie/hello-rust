fn main() {
    println!("Hello, world!");

    // References
    let s1 = String::from("rust");
    let len = calculate_length(&s1);
    println!("String '{}' length is {}.", s1, len);

    // borrowing
    let mut s2 = String::from("rsuter");
    change(&mut s2);
    println!("s2 value: {}", s2);
    let s3 = &mut s2;
    println!("s3: {}", s3);

    // let s4 = &mut s2;
    //println!("s3: {}, s4: {}", s3, s4)

    println!("\n# Listing 4-6");
    let s5 =  String::from("hello");
    change1(&s5);
    let mut s6 = String::from("hello");
    change2(&mut s6);
    println!("s6: {}", s6);
    let mut s7 = String::from("s7");
    let s8 = &mut s7;
    // println!("s8: {}", s8);
    let s9 = &mut s7;
    println!("s9: {}", s9);

    let reference_to_nothing = dangle();
}

fn calculate_length(str : &String) -> usize { // s is a reference to a string
    str.len()
}

fn change(str: &mut String) {
    str.push_str(", hello.");
}

fn change1(str: &String) {
    //str.push_str(", hello.");
}

fn change2(str: &mut String) {
    str.push_str(", hello.");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn dangle() -> String {
    let s = String::from("hello");
    s
}
