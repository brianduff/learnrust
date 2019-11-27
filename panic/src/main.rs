fn main() {
    test_panic_slice();
    test_panic_macro();
}

fn test_panic_slice() {
    let v = vec![1, 2, 3];
    v[99];
}

fn test_panic_macro() {
    panic!("Argh, it's all gone terribly pear shaped");
}
