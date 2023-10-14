use super::trait_::Drawable;
use super::struct_rectangle::Rectangle;

impl Drawable<u32> for Rectangle {
  fn check(&self) -> bool {
    self.dimension == None
  }

  fn draw(&self) {
    println!(
      "Рисуем прямоугольник шириной {} и высотой {}.",
      self.width, self.height
    )
  }

  fn area(&self) -> u32 {
    if self.check() {
      self.dimension.map_or(
        self.width * self.height, 
        |dimension| dimension.0 * dimension.1
      )
    } else {
      0
    }
  }

  fn area_result(&self) {
    println!(
      "Результат четырех сторон: {}",
      self.area()
    )
  }  
}
