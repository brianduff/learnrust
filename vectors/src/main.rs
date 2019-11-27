fn main() {
    test_vector_creation_and_dereferencing();
    test_vector_iteration();
    test_vector_of_enum_values();
}

fn test_vector_creation_and_dereferencing() {
    // Create a new vector. Needs a type hint, because it's empty
    let v: Vec<i32> = Vec::new();
    // This won't work, compiler doesn't know the type:
    //let v2 = Vec::new();

    // The vec macro creates a new vector with values:
    let v = vec![1, 2, 3];

    // The type inference tries harder than Java, e.g. we don't have to
    // specify a type here, because we're adding
    let mut v = Vec::new(); // compiler knows it's i32 because of subsequent push() calls
    v.push(5);
    v.push(6);
    v.push(42);

    // You can get an element by array index syntax:
    let third = v[2];
    // The book does this, which takes a reference
    let third_ref: &i32 = &v[2];
    println!("{}", third);

    // You can also use get(), which returns an Option
    match v.get(47) {
        Some(third) => println!("The 47th element is {}", third),
        None => println!("There is no 47th element"),
    }

    // This would panic at runtime
    // let huge = v[46];

    // Take a reference to an element prevents the vector from being written
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // This is not allowed because it's possible v.push will cause
    // the vector to be rellocated, meaning first is pointing at garbage
    //v.push(6);
    println!("The first element is {}", first);
}

fn test_vector_iteration() {
    let v = vec![100, 32, 58];
    // Simple iteration with for
    for i in &v {
        println!("{}", i);
    }

    // Mutation while iterating
    let mut v = v;
    for i in &mut v {
        // Note the dereferencing operator
        *i += 50;
    }
}


fn test_vector_of_enum_values() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
        
}
