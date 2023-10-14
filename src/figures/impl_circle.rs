use super::trait_::Drawable;
use super::struct_circle::Circle;

impl Drawable<f32> for Circle {
  fn check(&self) -> bool {
    self.radius != 0.0
  }

  fn draw(&self) {
    println!(
      "Рисуем круг с радиусом {}.",
      self.radius
    )
  }

  fn area(&self) -> f32 {
    self.radius * 2.0
  }

  fn area_result(&self) {
    println!(
      "Результат четырех сторон: {}",
      self.area()
    )
  }
}