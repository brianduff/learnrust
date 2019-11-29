use std::env;
use std::process;
// VSCode's rust support errors on this, but rustc is fine. Sigh.
// use minigrep::Config;
// This works
mod lib;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
