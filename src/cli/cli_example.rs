use std::{
  env, process
};

use super::{Config, run};

pub fn smth() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("\n -> Problem parsing arguments: {err}");
    process::exit(1);
  });

  if let Err(error) = run(config) {
    eprintln!("Application error: {error}");
    process::exit(1);
  };
}

// cargo run -- to src/cli/poem.txt > src/cli/output.txt