pub fn body() {
    println!("\nListing 8-7: Printing each element in a vector by iterating over the elements using a for loop");
    let v = vec![100, 32, 57];
    for i in &v { print!("{i} ") }
    println!();

    println!("\nListing 8-8: Iterating over mutable references to elements in a vector");
    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 { *i += 50 }
    println!("vector: {:?}", v1);
}