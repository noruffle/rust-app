use super::{Rectangle, Circle, Drawable};

pub fn _draw_figures() {
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