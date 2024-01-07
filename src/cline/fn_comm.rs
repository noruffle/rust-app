use std::{
  env, process
};

use super::{Config, run};

pub fn smth() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {err}");
      process::exit(1);
  });

  println!(
    "Searching for {}", 
    config.query);
  println!(
    "In file {}\n", 
    config.filepath);

  if let Err(error) = run(config) {
    println!(
      "Application error: {}",
      error);
    process::exit(1);
  };
}