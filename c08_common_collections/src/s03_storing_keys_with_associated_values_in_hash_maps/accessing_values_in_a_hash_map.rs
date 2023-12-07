use std::collections::HashMap;

pub fn body() {
    println!("\nListing 8-21: Accessing the score for the Blue team stored in the hash map");
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    for (key, value) in &scores { println!("{key}: {value}"); }
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team: {:?}, name: {:?}, score: {:?}", scores, team_name, score);
}