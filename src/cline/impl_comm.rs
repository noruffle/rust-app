use std::{fs, error::Error};
use super::Config;

impl Config {
  
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    
    if args.len() < 3 {
      return Err("\n>>> Command haven't enough arguments\n");
    }
    let query = args[1].clone();
    let filepath = args[2].clone();

    Ok(Config { query, filepath })
  }
  
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filepath)?;

  for line in search(&config.query, &contents) {
      println!("{line}");
  }

  Ok(())
}

pub fn search<'a>(query: &str, ctx: &'a str) -> Vec<&'a str> {

  let mut results = Vec::new();

  for line in ctx.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}