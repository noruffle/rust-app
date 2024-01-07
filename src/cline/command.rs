
#[cfg(test)]
mod tests {
  use super::super::search;

  #[test]
  fn commandline_result() {
    
    let (query, cx) = (
      "duct",
      "\
Rust:
safe, fast, productive.
Pick three."
    );

    assert_eq!(
      vec!["safe, fast, productive."], 
      search(query, cx)
    );
  }
}