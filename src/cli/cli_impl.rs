use std::{fs, error::Error, env};
use super::Config;

impl Config {
  
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    
    if args.len() < 3 {
      return Err("not enough arguments\n");
    }

    let (query, file_path, ignore_case) = (
      args[1].clone(),
      args[2].clone(),
      env::var("IGNORE_CASE").is_ok()
    );

    Ok(Config { 
      query, 
      file_path, 
      ignore_case 
    })
  }
  
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

  let cx = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &cx)
  } else {
    search(&config.query, &cx)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}

pub fn search<'a>(
  query: &str, 
  cx: &'a str
) -> Vec<&'a str> {

  let mut results = Vec::new();

  for line in cx.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a> (
  query: &str,
  cx: &'a str
) -> Vec<&'a str> {
  let (query, mut results) = (
    query.to_lowercase(),
    Vec::new()
  );

  for line in cx.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}