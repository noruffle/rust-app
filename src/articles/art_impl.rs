use super::*;

impl Summary for Anime {

  fn author(&self) -> String {
    format!(
      "@{}", 
      
      self.title
    )
  }
}

impl Summary for Comment {
  fn author(&self) -> String {
    format!(
      "@{}", 
      
      self.username
    )
  }

  fn summarize(&self) -> String {
    format!(
      "{}: {}",
      
      self.username,
      self.content,
    )
  }
}