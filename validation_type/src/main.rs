#[derive(Debug)]
pub struct Guess {
    // This is intentionally private, so it can't be changed without
    // going through the new() method's validation
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // From our guessing game earlier, we want to enforce that the number is between 1 and 100
    let g = Guess::new(75);

    // This will panic
    //let g = Guess::new(1003);

    // But oh dear, this won't. But that's just because we're inside the same module. Modules matter :)
    let mut g = Guess::new(20);
    g.value = 10010010;

    println!("A crazy guess: {:?}", g);

}
