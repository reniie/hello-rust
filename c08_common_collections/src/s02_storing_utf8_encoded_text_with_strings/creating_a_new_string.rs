pub fn body() {
    println!("\nListing 8-11: Creating a new, empty String");
    let mut s = String::new();
    println!("new string: {:?}", s);

    println!("\nListing 8-12: Using the to_string method to create a String from a string literal");
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("to_string s: {:?}", s);

    println!("\nListing 8-13: Using the String::from function to create a String from a string literal");
    let s = String::from("initial contents");
    println!("String::from s: {:?}", s);

    println!("\nListing 8-14: Storing greetings in different languages in strings");
    let helloA = String::from("السلام عليكم");
    let helloB = String::from("Dobrý den");
    let helloC = String::from("Hello");
    let helloD = String::from("שָׁלוֹם");
    let helloE = String::from("नमस्ते");
    let helloF = String::from("こんにちは");
    let helloG = String::from("안녕하세요");
    let helloH = String::from("你好");
    let helloI = String::from("Olá");
    let helloJ = String::from("Здравствуйте");
    let helloK = String::from("Hola");
    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", helloA, helloB, helloC, helloD, helloE, helloF, helloG, helloH, helloI, helloJ, helloK);

}