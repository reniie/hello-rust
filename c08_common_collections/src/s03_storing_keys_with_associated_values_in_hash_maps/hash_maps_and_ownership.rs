use std::collections::HashMap;

pub fn body() {
    println!("\nListing 8-22: Showing that keys and values are owned by the hash map once they’re inserted");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("field_name: {}, field_value: {}", field_name, field_value);
    println!("map is : {:?}.", map);
}