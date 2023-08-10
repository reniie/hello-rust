fn main() {
    println!("Hello, world!");

    println!("\nListing 5-2");
    let user = User {
        active: true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    };

    println!("\nListing 5-3");
    let mut user1 = User {
        active: true,
        username: String::from("user2"),
        email: String::from("someone@example.com"),
        sing_in_count: 1
    };
    user1.email = String::from("another@example.com");

    println!("\nListing 5-6");
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sing_in_count: user1.sing_in_count
    };

    println!("\nListing 5-7");
    let user22 = User {
        email: String::from("another@example.com"),
        ..user2
    };
}

// Listing 5-1
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64
}

// Listing 5-5
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username, // Listing 5-4  
        username,              // field init shorthand
        // email: email,       // Listing 5-4
        email,                 // field init shorthand
        sing_in_count: 1
    }
}

// Ownership of Struct Data
struct UserA {
    active: bool,
    username: &str,
    email: &str,
    sing_in_count: u64
}