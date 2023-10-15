use super::trait_::Summary;
use super::struct_::*;

/* pub fn notify(item: &impl Summary) {
  println!("Breaking news: {}", item.summarize())
}

pub fn notification<T: Summary + Display> (item_one: &T, item_two: &T) {
  println!(
    "Breaking news: {}\n,
    Also watch: {}\n", 
    
    item_one.summarize(), 
    item_two.summarize()
  )
}

pub fn some<T, U> (t: &T, u: &U) -> i32
where 
  T: Display + Clone,
  U: Clone + Debug,
{
  1
} */

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