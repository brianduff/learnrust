struct User {
    // We intentionally use String here, because the struct owns its data.
    // Using &str would require lifetimes, which we haven't covered yet.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    test_create_struct();
    test_tuple_structs();
}

fn test_create_struct() {
    let mut user1 = User {
        email: String::from("brian@dubh.org"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 25
    };

    println!("{}", user1.email);

    // To change a field, the instance must be mutable using let mut.
    // then all fields are mutable
    user1.email = String::from("changed@foo.bar");
    println!("{}", user1.email);

    let user2 = build_user_ugly(String::from("far@point.com"), String::from("far"));
    let user3 = build_user_pretty(String::from("far@point.com"), String::from("far"));

    // Here we create a new user with a different email / username, but use
    // struct update syntax to use the same value for other fields as user3
    let user4 = User {
        email: String::from("efef@efefef.ef"),
        username: String::from("efef"),
        ..user3
    };
}

// A function to build a user, the ugly way.
fn build_user_ugly(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// A function to build a user, the pretty way without repetition
fn build_user_pretty(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Dimension(i32, i32);

fn test_tuple_structs() {
    let black = Color(0, 0, 0);
    let small = Dimension(10, 10);

    println!("Height: {}", small.1);
}