fn main() {
    println!("Hello, world!");

    println!("\nListing 5-13:");
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect.area());

    println!("\nListing 5-14:");
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    println!("\nAssociated Functions:");
    let square = Rectangle::square(5);
    println!("square: {:#?}", square);
}

// Listing 5-13
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 { // &self is a reference to an instance of Rectangle
        self.width * self.height
    }
    // fn width(&self) -> bool {
    //     self.width > 0
    // }
    // Listing 5-15
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

