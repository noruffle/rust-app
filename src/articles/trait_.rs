pub trait Summary {
  fn author(&self) ->  String;
  fn summarize(&self) -> String {
    format!("Read more of {}", self.author())
  }
}