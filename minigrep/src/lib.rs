use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("{}", contents);

  Ok(())
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }
      Ok(Config {
          query: args[1].clone(),
          filename: args[2].clone(),
      })  
  }
}
