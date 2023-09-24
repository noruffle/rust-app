pub mod figures;

use figures::{Rectangle, Drawable};

fn main() {

  let rectangle = Rectangle {
    width: 4,
    height: 4,
    dimension: None,
  };

  rectangle.draw();
  rectangle.area_result();
}