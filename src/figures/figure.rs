pub trait Drawable {
  fn check(&self) -> bool;
  fn draw(&self);
  fn area(&self) -> u32;
  fn area_result(&self);
}