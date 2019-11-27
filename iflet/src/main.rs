fn main() {
    test_if_let();
}

fn test_if_let() {
    let some_value = Some(0u8);
    // A condition which will only be true if some_value has a value and it's 3.
    if let Some(3) = some_value {
        println!("three");
    }
    // It's sugar for:
    match some_value {
        Some(3) => println!("three"),
        _ => ()
    }
    // The else clause works like the _ case
    if let Some(3) = some_value {
        println!("three");
    } else {
        println!("Some other value.")
    }
    
}
