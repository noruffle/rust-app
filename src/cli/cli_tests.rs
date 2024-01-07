
#[cfg(test)]
mod tests {

  #[test]
  fn case_sensitive() {
    
    let (query, cx) = (
      "duct",
      "\
Rust:
safe, fast, productive.
Pick three."
    );

    assert_eq!(
      vec!["safe, fast, productive."], 
      super::super::search(query, cx)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      super::super::search_case_insensitive(query, contents)
    );
  }
}