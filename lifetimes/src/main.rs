fn main() {
    test_dangling_references();
    test_lifetimes();
    test_lifetime_simple_example();
    test_struct_with_reference();
}

fn test_dangling_references() {
    // This doesn't compile because we're trying to use a dangling reference
    // let r;

    // {
    //     let x = 5;
    //     // We get an error "x does not live long enough", because we're borrowing x, but it goes out of scope later.
    //     r = &x;
    // }

    // println!("r: {}", r);

    // Rust's borrow checker noticed that the lifetime of x is shorter than the lifetime of r.
}

fn test_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This won't compile because rust doesn't know whether the return value is borrowed from x or y, so
// it can't figure out the lifetime of these references.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// We use lifetime annotations to solve this
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// We declare generic lifetime parameters inside angle brackets just like generic types
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime_simple_example() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}


// But this is not ok. string2 doesn't live long enough. Rust knows this because we
// annotated the parameters and the return type with the same lifetime. Even though
// we can reason that it's safe (since we know it will return string1, and that exists
// in the outer scope), Rust's borrow checker can only rely on the lifetime annotations
// we've used.
// Doesn't compile:
// fn test_lifetime_out_of_scope() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// The lifetime parameter for return type must match at least one of the params.
// We can't return dangling references, so this doesn't compile:
// fn longest_dangle<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()   // this would be a dangling reference
// }

// Structs normally contain owned types, but we can make them hold references if we specify a lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_struct_with_reference() {
    let novel = String::from("anyone lived in a pretty how town (with up so floating many bells down)");
    let first_word = novel.split_whitespace().next().expect("Could not find first word");
    let important = ImportantExcerpt { part: first_word };

    // novel doesn't go out of scope until after the ImportantExcerpt goes out of scope, so we're good
    println!("Important excerpt: {}", important.part);
}

// There are some special cases ("lifetime elision rules") where the rust compiler allows you to skip
// lifetime annotations because these are common cases that are deterministic and well understood

// Rules for when there are no explicit annotations:
// 1. Each parameter gets its own (distinct) lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

// Eg. when we write:
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().expect("Could not find word")
}

// Rule 1 gives each parameter its own lifetime parameter:
fn first_word_1<'a>(s: &'a str) -> &str {
    s.split_whitespace().next().expect("Could not find word")
}

// and rule 2 makes the output lifetime parameter the same because there's only one parameter:
fn first_word_2<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().expect("Could not find word")
}


// Lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // This applies the third lifetime elision rule. The lifetime of self is assigned to all outputs
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


// The static lifetime - can live for the duration of the program
// e.g. all string literals are 'static