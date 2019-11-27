fn main() {
    test_first_word_without_slices();
    test_simple_slice_example();
    test_first_word_with_slices();
    test_array_slice();
}

fn test_first_word_without_slices() {
    // This works, but if s later changes, word will be wrong.
    let s = String::from("Hello there");
    let word = end_of_first_word_index(&s);
    println!("{}", word);
}

fn end_of_first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn test_simple_slice_example() {
    let s = String::from("Star Trek Discovery");
    let star = &s[0..4];
    let discovery = &s[10..];

    println!("{} {}", discovery, star);
}

fn test_first_word_with_slices() {
    // This works, but if s later changes, word will be wrong.
    let s = String::from("Hello there");
    let word = first_word(&s);

    // If we try to do something that will invalidate the slice, we'll
    // get a compile time (!) error E.g.
    // s.clear();

    println!("{}", word);
}

// The &str type signifies "string slice"
// We change the parameter to &str too so we can take either Strings or slices.
// In the book, they pass in [..], but there seems to be some kind of automatic
// coercion here. 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn test_array_slice() {
    let a = [1, 2, 3, 4, 5];
    // Surprisingly an out of bounds end specifier is ok.. I assume it does the right thing.
    let slice = &a[1..9];
}