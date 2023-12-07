use std::collections::HashMap;

pub fn body() {
    overwriting_a_value();
    adding_a_key_and_value_only_if_a_key_is_not_present();
    updating_a_value_based_on_the_old_value();
}

fn overwriting_a_value() {
    println!("\nListing 8-23: Replacing a value stored with a particular key");
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 50);
    println!("scores: {:?}.", scores);
}

fn adding_a_key_and_value_only_if_a_key_is_not_present() {
    println!("\nListing 8-24: Using the entry method to only insert if the key does not already have a value");
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);
    println!("scores: {:?}.", scores);
}

fn updating_a_value_based_on_the_old_value() {
    println!("\nListing 8-25: Counting occurrences of words using a hash map that stores words and counts");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value for this key
        // if the key exists, and inserts the key with the default value if it doesn’t
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map: {:?}.", map);
}