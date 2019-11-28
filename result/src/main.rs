use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    test_match_error_kind();
    test_basic_result_handling();
    test_unwrap();
    test_propagating_errors();
}

fn test_basic_result_handling() {
    // File::open returns a Result<File>
    let f = File::open("hello.txt");

    // We can handle it with match
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Unable to open file: {:?}", error)
        },
    };
}

fn test_match_error_kind() {
    // File::open returns a Result<File>
    let f = File::open("hello.txt");

    // We can handle it with match. There's lots of ugly here, this
    // will be cleaner when we learn how to use closures.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => panic!("Problem opening file {:?}", other_error),
        },
    };
}

fn test_unwrap() {
    // unwrap automatically does what basic_result_handling above does: it'll
    // return the success if the call was successful, or panic if it failed.
    let f = File::open("hello.txt").unwrap();

    // Expect is similar, but lets us specify an error message.
    let f = File::open("hello.txt").expect("Oh noes! hello.txt doesn't exist");
}

fn test_propagating_errors() {
    let username = read_username_from_file();
    match username {
        Ok(username) => println!("The username is {}", username),
        Err(error) => panic!("Couldn't read username file: {:?}", error)
    }
    let username = read_username_from_file_nicer();
    match username {
        Ok(username) => println!("The username is {}", username),
        Err(error) => panic!("Couldn't read username file: {:?}", error)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s.trim().to_string()),
        Err(e) => Err(e),
    }
}

// This version is nicer, it uses the ? operator which behaves like the matches in the previous
// function.
fn read_username_from_file_nicer() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

// We can chain the calls to make it even more brief.
fn read_username_from_file_even_nicer() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

fn read_username_from_file_nicest() -> Result<String, io:Error> {
    // The nicest of all, but we can't trim.
    fs::read_to_string("username.txt");
}