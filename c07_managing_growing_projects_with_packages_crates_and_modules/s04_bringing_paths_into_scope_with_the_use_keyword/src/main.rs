// Listing 7-14: Bringing HashMap into scope in an idiomatic way
use std::collections::HashMap;
// Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.
use std::fmt;
use std::io;
// Listing 7-16: Renaming a type when it’s brought into scope with the as keyword
use std::fmt::Result;
use std::io::Result as IoResult;
// Listing 7-19: Two use statements where one is a subpath of the other
//use std::io;
//use std::io::Write;
// Listing 7-20: Combining the paths in Listing 7-19 into one use statement
//use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    println!("\nListing 7-14: Bringing HashMap into scope in an idiomatic way");
    let mut map = HashMap::new();
    map.insert(1,2);
    println!("map: {:?}", map);

    // Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.
    //fn function1() -> fmt::Result {}
    //fn function2() -> io::Result<()> {}
    // Listing 7-16: Renaming a type when it’s brought into scope with the as keyword
    //fn function3() -> Result {}
    //fn function4() -> IoResult<()> {}
}
