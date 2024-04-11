pub fn body() {
    panic!("crash and burn");
}

pub fn using_a_panic_backtrace() {
    println!("\nListing 9-1: Attempting to access an element beyond the end of a vector, which will cause a call to panic!");
    let v = vec![1, 2, 3];
    v[99];
}