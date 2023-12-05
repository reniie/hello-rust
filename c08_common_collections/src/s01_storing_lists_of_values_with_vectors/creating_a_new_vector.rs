//fn body() {
pub fn body() {
    println!("\nListing 8-1: Creating a new, empty vector to hold values of type i32");
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    println!("\nListing 8-2: Creating a new vector with initial values");
    let v1 = vec![1,2,3];
    println!("v1: {:?}", v1);
}