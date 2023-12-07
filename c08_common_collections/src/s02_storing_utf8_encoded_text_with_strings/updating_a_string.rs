pub fn body() {
    println!("\nListing 8-15: Appending a string slice to a String using the push_str method");
    let mut s = "hello".to_string();
    println!("string before push_str: {}", s);
    s.push_str(", world!");
    println!("string after push_str: {}", s);

    println!("\nListing 8-16: Using a string slice after appending its contents to a String");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");    

    println!("\nListing 8-17: Adding one character to a String value using push");
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    println!("\nListing 8-18: Using the + operator to combine two String values into a new String value");
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + "," + &s2 + "!";
    println!("result is {:?}.", s3);

    println!("\nListing 8-19: Attempting to use indexing syntax with a String");
    let s1 = String::from("hello");
    //println!("index 1 of string {:?} is {:?}.", s1, s1[1]);

}