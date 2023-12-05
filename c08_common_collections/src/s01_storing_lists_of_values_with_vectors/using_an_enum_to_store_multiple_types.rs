pub fn body() {
    println!("\nListing 8-9: Defining an enum to store values of different types in one vector");
    #[derive(Debug)]
    enum SpreadsheetCell { Int(i32), Float(f64), Text(String) }
    let row = vec![ SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("blue")), SpreadsheetCell::Float(10.12) ];
    println!("row: {:?}", row);
}