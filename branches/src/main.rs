fn main() {
    test_simple_if();
    test_if_as_an_expr();
    test_return_from_loop();
    test_while();
    test_for();
}

fn test_for() {
    // Iterate a collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {}", element);
    }

    // Just do a fixed number of iterations
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}

fn test_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}

// You can return values from loops
fn test_return_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// Ternary like if expressions
fn test_if_as_an_expr() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is {}", number);
}

// Simple if statements
fn test_simple_if() {
    let number = 100;

    if number < 5 {
        println!("condition was true");
    } else if number % 2 == 0 {
        println!("number is even");
    } else {
        println!("condition was false");
    }
}
