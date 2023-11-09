use super::enum_::Color;
pub struct Inventory {
  pub shirts: Vec<Color>
}

pub struct User<'a> {
  pub name: &'a str,
}