// Simple enum with no data
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Enum values can contain data
enum IpAddrKindWithAddr {
    V4(String),
    V6(String),
}

// The types of the data can be different
enum IpAddrKindWithDifferentTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ip4Addr {}
struct Ip6Addr {}

// They can even be structs
enum IpAddrUsingStructs {
    V4(Ip4Addr),
    V6(Ip6Addr)
}

enum Message {
    Quit,
    // This is an anonymous struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can also add methods to enums
impl Message {
    fn call(&self) {
        println!("You rang? :)");
    }
}

fn main() {
    test_using_enums();
    test_using_option();
}

fn test_using_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Enums: {:?} {:?}", four, six);

    let home = IpAddrKindWithAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithAddr::V6(String::from("::1"));

    let home2 = IpAddrKindWithDifferentTypes::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn test_using_option() {
    // Part of std library:
    // enum Option<T> {
    //   Some(T),
    //   None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use none, we have to tell the compiler what type it is.
    let absent_number: Option<i32> = None;
}