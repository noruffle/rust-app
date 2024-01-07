use super::trait_::Area;
use super::struct_circle::Circle;

impl Circle<'_> {
  pub fn new(radius: f32) -> Self {
    Circle { title: "Circle", radius }
  }
}

impl Area for Circle<'_> {

  fn check(&self) -> bool {
    self.radius != 0.0
  }

  fn area(&self) {
    println!(
      "Радиус круга: {}",
      self.radius * 2.0
    )
  }
}
