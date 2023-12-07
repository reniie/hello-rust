mod s01_storing_lists_of_values_with_vectors;
mod s02_storing_utf8_encoded_text_with_strings;
mod s03_storing_keys_with_associated_values_in_hash_maps;

pub use s01_storing_lists_of_values_with_vectors::*;
pub use s02_storing_utf8_encoded_text_with_strings::*;
pub use s03_storing_keys_with_associated_values_in_hash_maps::*;

fn main() {
    println!("Hello, world!");

    println!("\n#8.1# Storing Lists of Values with Vectors");
    // creating_a_new_vector::body();
    // updating_a_vector::body();
    // reading_elements_of_vectors::body();
    // iterating_over_the_values_in_a_vector::body();
    // using_an_enum_to_store_multiple_types::body();

    println!("\n#8.2# Storing UTF-8 Encoded Text with Strings");
    //creating_a_new_string::body();
    //updating_a_string::body();

    println!("\n#8.3# Storing Keys with Associated Values in Hash Maps");
    //creating_a_new_hash_map::body();
    //accessing_values_in_a_hash_map::body();
    //hash_maps_and_ownership::body();
    updating_a_hashmap::body();
    
}