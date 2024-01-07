use super::*;
pub struct Inventory {
  pub shirts: Vec<Shirt>
}

impl Inventory {
  pub fn giveaway(&self, user_preference: Option<Shirt>) -> Shirt {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> Shirt {
    let mut num_red: i32 = 0;
    let mut num_blue: i32 = 0;

    for shirt in &self.shirts {
      match shirt {
        Shirt::Red => num_red += 1,
        Shirt::Blue => num_blue += 1,
      }
    }

    if num_red > num_blue {
      Shirt::Red
    } else {
      Shirt::Blue
    }
  }
}