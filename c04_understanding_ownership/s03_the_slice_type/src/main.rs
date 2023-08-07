fn main() {
    println!("Hello, world!");

    // Listing 4-7
    let mut s = String::from("hello world!");
    let word = first_word(&s); // word will get the value 5
    println!("[ before clear ] word: {}", word);
    s.clear();                      // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("[ after  clear ] word: {}", word);
    let s_slice = String::from("hello world!");
    let word_slice = first_word_slice(&s_slice);
    println!("[ before clear ] word_slice: {}", word_slice);
    // s_slice.clear();
    // println!("[ after  clear ] word_slice: {}", word_slice);

    // Listing 4-9
    let my_string = String::from("hello world");
    // `first_word_slice_b` works on slices of `String`s, whether partial or whole
    let word = first_word_slice_b(&my_string[0..6]);
    let word = first_word_slice_b(&my_string[..]);
    // `first_word` also works on reference to `String`s, which are equivalent ro whole slices of `String`s
    let word = first_word_slice_b(&my_string);
    let my_string_literal = "hello world";
    // `first_word_slice_b` works on slices of string literals, whether partial or whole
    let word = first_word_slice_b(&my_string_literal[0..6]);
    let word = first_word_slice_b(&my_string_literal[..]);
    // Because string literals *are* string slices already, this works too, without the slice syntax!
    let word = first_word_slice_b(my_string_literal);
} 

// Listing 4-7
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Listing 4-9
fn first_word_slice_b(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
