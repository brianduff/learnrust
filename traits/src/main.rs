use std::fmt::Display;

fn main() {
    test_call_trait_method();
    test_use_trait_parameter();
}

// Traits are kind of like interfaces
pub trait Summary {
    // Just like an interface you can have methods without bodies:
    fn summarize_author(&self) -> String;

    // And it's also possible to provide a default implementation:
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how we provide an implementation of a trait for a struct
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Note: we can only provide a trait implementation for structs within the same crate.
// But it's ok to add a new trait in a different crate that adds methods to a struct in
// a different crate, so you can extend traits to existing structs, which is very useful
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        String::from(&self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn test_call_trait_method() {
    let tweet = Tweet {
        username: String::from("someone"),
        content: String::from("this is a cool tweet"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn test_use_trait_parameter() {
    let news_article = NewsArticle {
        headline: String::from("Shocking defeat!"),
        location: String::from("Gurnigsbergintown"),
        author: String::from("Jacklaton Sporkin"),
        content: String::from("It was the best of times, it was the worst of times..."),
    };

    notify(news_article);
}

// Here's a function that takes something that implements the Summary trait
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// There's a longer syntax for this called trait bound. This takes some T that implements
// Summary.
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// The trait bound syntax is useful when we have > 1 parameter that implements the trait
pub fn notify_multiple<T: Summary>(first: T, second: T) {
    println!("Breaking news! {} AND {}", first.summarize(), second.summarize());
}

// We can require multiple trait bounds using +

pub trait MyDisplay {
    fn display(&self) -> String {
        String::from("The default impl of display")
    }
}

impl MyDisplay for Tweet {
    // It's ok to not provide an implementation; Tweet implements the trait but uses the default
}

// item must implement both Summary and Display
pub fn notify_displayable(item: impl Summary + MyDisplay) {
    println!("{} {}", item.summarize(), item.display());
}

// can also use trait bound syntax
pub fn notify_displayable2<T: Summary + MyDisplay>(item: T) {
    println!("{} {}", item.summarize(), item.display());
}

// This can get ugly when there are lots of bounds
pub fn notify_crazy<T: MyDisplay, U: MyDisplay + Summary, V: Summary>(t: T, u: U, v: V) {
    println!("{}{}{}", t.display(), u.summarize(), v.summarize());
}

// So there's a where clause feature
pub fn notify_less_crazy<T, U, V>(t: T, u: U, v: V)
    where T: MyDisplay,
          U: MyDisplay + Summary,
          V: Summary
{
    println!("{}{}{}", t.display(), u.summarize(), v.summarize());        
}

// We can return things that implement traits
pub fn returns_summary() -> impl Summary {
    Tweet {
        username: String::from("crabbie"),
        content: String::from("I'm confused"),
        reply: false,
        retweet: false,
    }
}

// BUT you can't return more than one type this way for impl reasons (eugh)
// This won't compile:
// pub fn returns_summary(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("crabbie"),
//             content: String::from("I'm confused"),
//             reply: false,
//             retweet: false,
//         }    
//     } else {
//         NewsArticle {
//             headline: String::from("crabbie"),
//             location: String::from("Ekaterin"),
//             content: String::from("Egads"),
//             author: String::from("Pokemala"),
//         }
//     }
// }

struct Pair<T> {
    first: T,
    second: T,
}

// The new method is implemented for all Pairs
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self {
            first, second
        }
    }
}

// But the cmp_display method is only implemented for Pairs that implement Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is first = {}", self.first);
        } else {
            println!("The largest member is second = {}", self.second);
        }
    }
}

