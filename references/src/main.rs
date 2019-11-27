fn main() {
    test_length_with_references();
    test_mutate_string_reference();
    test_mutable_and_immutable_borrow();
}

fn test_length_with_references() {
    let s1 = String::from("hello");
    // Pass a reference to s1.
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);
}

// Calculate length takes a reference to a String
// s will not be dropped when it goes out of scope
// Referred to as "borrowing"
fn calculate_length(s: &String) -> usize {
    // If we try to mutate the string, we'll get an error
    // s.push_str("I am evilly mutating");
    s.len()
}

fn test_mutate_string_reference() {
    let mut s = String::from("Hello");
    add_world(&mut s);
    println!("{}", s);
    // The second borrow will fail because we're not allowed to have >1 mutable borrow in the same
    // scope (presumably, the one in the add_world call is not considered in the same scope).
    // let r2 = &mut s;
    // let r3 = &mut s;
    // println!("{} {}", r2, r3);
}

fn test_mutable_and_immutable_borrow() {
    let mut s = String::from("hello");
    let r1 = &s;
    // Can't take a mutable reference (in r2) when we already have an immutable one (in r1)
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // This is ok
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // No more usages of r1 and r2 after this


    // So this is ok
    let r3 = &mut s;
    println!("{}", r3);

    // But if I tried to use r2 here, the "r3 = &mut s;" line above would break.
    // println!("{}", r2);
}

// This won't work because s falls out of scope at the end of the function.
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn add_world(s: &mut String) {
    s.push_str(" brian!");
}
