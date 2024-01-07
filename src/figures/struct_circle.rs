pub struct Circle<'a> {
  pub title: &'a str,
  pub radius: f32,
}

#[cfg(test)]
mod tests {
  
  use crate::figures::Area;

use super::Circle;

  #[test]
  fn circle_area () {

    let radius = 25.0;
    Circle::new(radius).area();
    
  }

}