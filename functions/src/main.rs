fn main() {
    println!("Hello, world!");
    another_function(5, 10);

    let x = 5;
    // Between the {} is a block that evaluates to 4
    let y = {
        let x = 3;
        x + 1  // Note: no semicolon. This is an expression, not a statement
    };

    println!("The value of y is {}", y);

    let x = five();
    println!("The value of x is {}", x);

    test_plus_one();
}

// Types must be specified for all function parameters.
fn another_function(x: i32, y: i32) {
    println!("x and y are {} and {}", x, y);
}

// Specify return type with -> 
fn five() -> i32 {
    5  // No semicolon. Expression!
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test_plus_one() {
    let x = plus_one(5);
    println!("The value of x is {}", x);
}
