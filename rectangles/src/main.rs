
// An annotation to opt in to debug info for our struct.
// This is needed to make the {:?} format specifier work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// You can have multiple impl blocks...
impl Rectangle {
    // This is a "method", defined in a struct, first param is always self.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an "associated function", it's not a method, but associated with the struct
    // String::from was an example
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(20);
    println!("rect4 is {:?}", rect4);
}
