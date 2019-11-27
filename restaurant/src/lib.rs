// You can use curlies in use statements to use multiple:
// Equivalent to
//   use std::cmp::Ordering;
//   use std::cmp::io;
use std::{cmp::Ordering, io};

// You can use "self" to refer to the current path:
use std::collections::{self, HashMap};

// Globs are also supported
use std::alloc::*;

mod front_of_house {

    // The front of house version of a table
    pub struct Table {}

    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // The pub designator applies to the struct itself
    pub struct Breakfast {
        // Each member needs to be specifically made pub if necessary
        pub toast: String,
        // This struct item is private
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // The back of house version of a table
    pub struct Table {}

    // By contrast, pub on an enum is the enum and all values
    pub enum Appetizer {
        Soup,
        Salad,
    }    

    fn fix_incorrect_order() {
        cook_order();
        // This is how we access other mods from this mod
        super::front_of_house::serving::serve_order();
        // Or we could have done it with an absolute path,
        // but this is more brittle if we move things around (supposedly)
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// This brings hosting into scope so we don't have to qualify it.
use crate::front_of_house::hosting;
// We could have done this with a relative path
//use front_of_house::hosting;

// We could also do this to bring a specific function into scope, but it's
// unidiomatic
use crate::front_of_house::hosting::add_to_waitlist;

// It's idiomatic to use this for structs, enums, e.g
use back_of_house::Breakfast;

// If there's a name clash, we can use "as" to distinguish
use front_of_house::Table as FrontTable;
use back_of_house::Table as BackTable;

pub fn eat_at_restaurant() {
    // Absolute path to module
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // We can do this because of the "use" statement above
    hosting::add_to_waitlist();

    // The second use lets us do this, but it's considered unidiomatic
    // BAD
    add_to_waitlist();

    // Order a summer breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Because of the struct use, we can also do:
    let mut another_meal = Breakfast::summer("Rye");

    // Change the toast type
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // We can't do this because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // Front of house vs back of house tables
    let front_table = FrontTable {};
    let back_table = BackTable {};
}