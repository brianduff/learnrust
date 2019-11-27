fn main() {
    test_simple_mutable_string();
    test_use_value_after_move();
    test_clone();
    test_ownership();
    test_ownership_with_return_values();
}

fn test_ownership_with_return_values() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("I'm going through a function");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // A horrible way to get the length of a string and still be able to use the string
    // afterwards using a tuple.
    let (s4, len) = calculate_length(s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// This function transfers ownership of the new string to the function that called it.
fn gives_ownership() -> String {
    String::from("I was returned from a function")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn test_ownership() {
    let s = String::from("hello");
    take_ownership(s);
    // This won't compile because we no longer own s.
    // println!("{}", s);

    let x = 5;
    make_copy(x);
    // This is ok, because i32 is Copy
    println!("{}", x);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn test_clone() {
    // We deep copy the string
    let mut s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    s1.push_str(" Moar stuff");
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn test_use_value_after_move() {
    let s1 = String::from("hello");

    // This is a "move" rather than a copy. New stack frame allocated with ptr, len, capacity pointing to existing
    // heap storage of the string, the original s1 is no longer usable.
    let s2 = s1;

// The compiler will complain about this, because s1 is no longer valid
//    println!("{}, world!", s1);
}

fn test_simple_mutable_string() {
    // Create a possibly mutable string.
    let mut s = String::from("Hello");

    s.push_str(" Universe");

    println!("{}", s);
}
