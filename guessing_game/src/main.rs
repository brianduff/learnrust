use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // An infinite loop
    loop {

        println!("Please input your guess.");

        // kind of like a static method call
        let mut guess = String::new();

        // &mut guess - a mutable reference to guess
        // read_line returns an io::Result, we call the expect() method
        // to assert success on this, and crash otherwise.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // We can shadow guess with a new variable.
        // Need trim() because guess contains the newline
        // The colon guess: u32 is specifying a type for guess.
        // Rust will also guess the type of secret_number as u32 based
        // on this.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I'm expecting a number, dude!");
                continue;
            },
        };
        
        println!("You guessed {}", guess);

        // Match is kind of like a switch, made up of "arms"
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // exit the infinite loop
            }
        }
    }
}
