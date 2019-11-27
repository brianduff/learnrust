fn main() {
    test_creating_strings();
    test_appending();
    test_concat();
    test_indexing();
}

fn test_creating_strings() {
    // Create a new empty String
    let mut s = String::new();

    // Create a String from a literal
    let data = "some string value";
    let s = data.to_string();

    // We can also just call it directly on the literal
    let s = "hello there".to_string();

    // This is equivalent to what we've been using so far:
    let s = String::from("well, hi");
}

fn test_appending() {
    let mut s = String::from("Jean-Luc");
    // This takes a string slice, otherwise it'd take ownership,
    // and that'd be annoying.
    s.push_str(" Picard");
    println!("{}", s);

    // Push takes a char
    s.push('!');
    println!("{}", s);
}

fn test_concat() {
    let s1 = String::from("William ");
    let s2 = String::from("Riker");
    let s3 = s1 + &s2;
    // Behind the scenes this uses something like
    //   fn add(self, s: &str) -> String
    // s2 is deref co-erced from String to str
    // We can't use s1 any more, but s2 is ok.

    // You can concat multiple strings
    let s1 = String::from("William ");
    let s4 = String::from(" engage!");
    let s = s1 + "-" + &s2 + "-" + &s4;

    println!("Here: {}", s);

    // But it's cleaner with format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

}

fn test_indexing() {
    let s1 = String::from("hello");
    // Can't do this because reasons
    // let h = s1[0];

    // Rust seems to be quite primitive OOB compared to java for
    // utf-8 string handling. It models things as vectors as bytes
    // internally, so multibyte weirdness ensues:
    let s1 = String::from("我是苏格兰人");
    let len = s1.len();
    // Here, Java would say 6, but Rust says 18 (the number of bytes)
    println!("The length of {} is {}", s1, len);

    // We also can't use slices to get character parts, e.g. this
    // code will panic at runtime
    // let scotland = &s1[2..5];
    // println!("Ah, you're from {}", scotland);

    // To treat the string as chars, you use chars()
    for c in s1.chars() {
        println!("{}", c);
    }

    // If you want to care about bytes:
    for b in s1.bytes() {
        println!("{}", b);
    }

    // If you want the char length, you do this. Note that count() is O(N)
    let len = s1.chars().count();
    println!("The string is {} characters long", len);

    // You still can't slice though:
    // Basically, getting substrings is a horrible poo-hole in Rust
    // compared to java. See this enthusiastic debate about how to do it...:
    // https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/9
    // let chars = s1.chars();
    // let scotland = &chars[2..5];
    // println!("Ah, you're from {}", scotland);
}