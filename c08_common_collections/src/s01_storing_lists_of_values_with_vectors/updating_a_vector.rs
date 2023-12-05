pub fn body(){
    println!("\nListing 8-3: Using the push method to add values to a vector");
    let mut v = Vec::new();
    println!("before push, v: {:?}", v);
    v.push(5); v.push(6); v.push(7); v.push(8);
    println!(" after push, v: {:?}", v);
}