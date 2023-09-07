#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}

// Listing 6-1
#[derive(Debug)]
struct IpAddrO {
    kind: IpAddrKind,
    address: String
}

//  attach data to each variant of the enum directly
#[derive(Debug)]
enum IpAddr{
    V4(String), V6(String)
}

// each variant can have different types and amounts of associated data
#[derive(Debug)]
enum IpAddrB {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Listing 6-2
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("Message.call: {:?}", self)
    }
}

// The Option Enum and Its Advantages Over Null Values
enum Option<T> {
    None,
    Some(T)
}

fn main() {
    println!("Hello, world!");

    println!("\n---- enum IpAddrKind ----");
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;
    route(four);
    route(six);

    println!("\n---- Listing 6-1 ----");
    let home = IpAddrO {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddrO {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("homd: {:?}, lookback: {:?}", home, loopback);

    println!("\n----  attach data to each variant of the enum directly ----");
    let home1 = IpAddr::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr::V6(String::from("::1"));
    println!("homd: {:?}, lookback: {:?}", home1, loopback1);

    println!("\n----  each variant can have different types and amounts of associated data ----");
    let homeB = IpAddrB::V4(127, 0, 0, 1);
    let loopbackB = IpAddrB::V6(String::from("::1"));
    println!("homd: {:?}, lookback: {:?}", homeB, loopbackB);

    println!("\n---- Listing 6-2 ----");
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("\n---- The Option Enum and Its Advantages Over Null Values ----");
    let some_number = Some(5);
    let some_char = Some('a');
    // let absent_number: Option<i32> = None;
}