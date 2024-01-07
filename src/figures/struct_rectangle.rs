pub struct Rectangle<'a> {
  pub title: &'a str,
  pub dimension: Option<(u32, u32)>
}

#[cfg(test)]
mod tests {
  
  use crate::figures::Area;

use super::Rectangle;

  #[test]
  fn rectangle_area () {
    
    let dimension = Some((200, 200));
    Rectangle::new(dimension).area();

  }
  
}