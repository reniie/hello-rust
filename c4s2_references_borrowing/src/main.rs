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

    // let s4 = &mut s2;
    //println!("s3: {}, s4: {}", s3, s4)
}

fn calculate_length(str : &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str(", hello.");
}