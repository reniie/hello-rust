pub fn body() {
    println!("\nListing 8-4: Using indexing syntax or the get method to access an item in a vector");
    let v = vec![1,2,3,4,5];
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    println!("\nListing 8-5: Attempting to access the element at index 100 in a vector containing five elements");
    let v = vec![1,2,3,4,5];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("\nListing 8-6: Attempting to add an element to a vector while holding a reference to an item");
    let mut v1 = vec![1,2,3,4,5];
    // let first = &v1[0];
    // v1.push(6);
    // println!("The first element is: {first}");
}