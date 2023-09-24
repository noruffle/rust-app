pub mod figures;

use figures::{Rectangle, Circle, Drawable};

fn main() {

  let rectangle = Rectangle {
    width: 4,
    height: 4,
    dimension: None,
  };

  let circle = Circle {
    radius: 5.0,
  };

  rectangle.draw();
  rectangle.area_result();

  circle.draw();
  circle.area_result();
}