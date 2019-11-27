#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    test_match_with_enum();
    test_match_with_option();
}

fn test_match_with_enum() {
    println!("The value of a dime is {}c", value(Coin::Dime));
    value(Coin::Quarter(UsState::Alaska));
}

fn value(coin: Coin) -> u8 {
    match coin {
        // An "arm" with a pattern => code (expression)
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Longer code blocks have curlies. We capture values with brackets
        Coin::Quarter(state) => {
            println!("You're wealthy, you have a quarter from {:?}", state);
            25
        }
    }
}

fn test_match_with_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}