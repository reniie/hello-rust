pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        //fn seat_at_table() {}
    }
    mod serving {
        //fn take_order() {}
        //fn serve_order() {}
        //fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Listing 7-9: A struct with some public fields and some private fields
    println!("\n ---- Listing 7-9: A struct with some public fields and some private fields ----");
    // Order a breakfast in the summer whit Rye toast
    let mut meal = back_of_house::BreakFast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Listing 7-10: Designating an enum as public makes all its variants public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// Listing 7-8: Calling a function using a relative path starting with super
fn deliver_order() {}
mod back_of_house {
    fn fix_incorrent_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
    
    // Listing 7-9: A struct with some public fields and some private fields
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String
    }
    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // Listing 7-10: Designating an enum as public makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad
    }
}
