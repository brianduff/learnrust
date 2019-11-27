fn main() {
    println!("Slower fib:");
    for i in 0..45 {
        println!("fib({}) = {}", i, slow_fib(i));
    }

    println!("Faster fib:");
    for i in 0..45 {
        println!("fib({}) = {}", i, faster_fib(i));
    }
}

// Slightly less slow dynamic programming fibonacci
fn faster_fib(n: i16) -> i64 {
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

// Slow, cute recursive fibonacci number
fn slow_fib(n : i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n-1) + slow_fib(n-2)
    }
}