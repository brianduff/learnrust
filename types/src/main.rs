fn main() {
    // TUPLES
    let tup: (bool, i32) = (true, 25);

    // "Destructuring" the tup to get its values
    let (the_boolean, the_value) = tup;
    println!("The boolean is {}, the value is {}", the_boolean, the_value);

    // Directly accessing tuple values:
    println!("The boolean is {}, the value is {}", tup.0, tup.1);

    // ARRAYS
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    // You can specify the type explicitly with [type; size]
    let pi: [i32; 5] = [3, 1, 4, 1, 5];

    // You can create an array with n of the same value v with [v; n]
    let rep = [1; 10]; // Same as [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

    // Get array elements the usual way.
    println!("Favorite day: {}", days[2]);
}
