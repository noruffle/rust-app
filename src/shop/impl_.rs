use super::enum_::Color;
use super::struct_::{Inventory};

impl Inventory {
  pub fn giveaway(&self, user_preference: Option<Color>) -> Color {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> Color {
    let mut num_black: i32 = 0;
    let mut num_white: i32 = 0;

    for color in &self.shirts {
      match color {
        Color::Black => num_black += 1,
        Color::White => num_white += 1,
      }
    }

    if num_black > num_white {
      Color::Black
    } else {
      Color::White
    }
  }

}
