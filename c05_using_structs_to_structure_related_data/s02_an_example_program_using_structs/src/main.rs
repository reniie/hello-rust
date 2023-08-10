fn main() {
    println!("Hello, world!");

    println!("\nListing 5-8");
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width, height));

    println!("\nListing 5-9");
    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuples(rect));

    println!("\nListing 5-10");
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area_structs(&rect1));

    println!("\nListing 5-11, 5-12");
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    let scale = 3;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect2);
}

// Listing 5-8
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Listing 5-9
fn area_tuples(dimensions: (u32, u32)) -> u32  {
    dimensions.0 * dimensions.1
}

// Listing 5-12
#[derive(Debug)]
// Listing 5-10
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}