fn main() {
    println!("Hello, world!");

    println!("\n----- Listing 6-3 -----");
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_coins(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => {
                println!("Not so lucky (nickel)");
                5
            },
            Coin::Dime => {
                println!("Not so lucky (dime)");
                10
            },
            Coin::Quarter => {
                println!("Not so lucky (quarter)");
                15
            },
        }
    }

    println!("\n----- Listing 6-4 -----");
    #[derive(Debug)]
    enum UsStata {
        Alabama,
        Alaska
    }
    enum CoinNew {
        Penny,
        Nickel,
        Dime,
        Quarter(UsStata),
    }
    fn value_in_cents(coin: CoinNew) -> u8 {
        match coin {
            CoinNew::Penny => 1,
            CoinNew::Nickel => 5,
            CoinNew::Dime => 10,
            CoinNew::Quarter(state) => {
                println!("State quarter from {:?}!", state); 
                25
            }
        }
    }
    value_in_cents(CoinNew::Quarter(UsStata::Alaska));

    println!("\n----- Listing 6-5 - Matching with Option<T> -----");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
             None => None,
            Some(i) => Some(i+1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    println!("\n----- Catch-all Patterns and the _ Placeholder -----");
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _ => reroll(),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}
