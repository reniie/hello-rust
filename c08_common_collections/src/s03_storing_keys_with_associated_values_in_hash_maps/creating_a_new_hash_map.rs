use std::collections::HashMap;

pub fn body() {
    println!("\nListing 8-20: Creating a new hash map and inserting some keys and values");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);
    println!("scores is {:?}.", scores);
}