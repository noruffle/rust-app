pub trait Drawable<T> {
  fn check(&self) -> bool;
  fn draw(&self);
  fn area(&self) -> T;
  fn area_result(&self);
}