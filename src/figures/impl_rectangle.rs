use super::trait_::Area;
use super::struct_rectangle::Rectangle;

impl Rectangle<'_> {
  pub fn new(dimension: Option<(u32, u32)>) -> Self {
    Rectangle { title: "Rectangle", dimension }
  }

  pub fn _draw(&self) {
    let height = self.dimension.unwrap().0;
    let weight = self.dimension.unwrap().1;
  
    println!(
      "Рисуем прямоугольник шириной {} и высотой {}.",
      weight, height
    )
  }
}

impl Area for Rectangle<'_> {
  fn check(&self) -> bool {
    self.dimension != None
  }

  fn area(&self) {
    if self.check() {
      let result = self.dimension.map(
        |dimension| 
        
        dimension.0 * dimension.1
      ).unwrap();

      println!(
        "Результат четырех сторон: {}",
        result
      )
    }
  }
}