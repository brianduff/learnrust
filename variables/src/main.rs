// Must specify a type for constants.
const MAX_POINTS: u32 = 100_000;

fn main() {

    // IMMUTABLE VARIABLES
    let x = 5;
    println!("The value of x is: {}", x);
    // Compiler error because vars are immutable by default
    // ERROR: x = 6;
    println!("The value of x is: {}", x);

    // SHADOWING
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // Cool example of changing type:
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);

    // But this isn't allowed, because the variable is mutable:
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
