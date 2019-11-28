fn main() {
    test_generic_struct(); 
}

// A generic function that takes a reference to an array of T and returns a T
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Structs can also use generic types:
struct Point<T> {
    x: T,
    y: T
}

fn test_generic_struct() {
    let integer_point = Point{ x: 5, y: 10 };
    let float_point = Point{ x: 1.0, y: 1.0 };
}

// Enums also
enum MyResult<T, E> {
    Ok(T),
    Err(E)
}

// The methods in impls of a struct can use the generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}