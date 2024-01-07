use super::{Inventory, Shirt};

pub fn store () {
  let store = Inventory {
    shirts: vec![
      Shirt::Blue,
      Shirt::Red,
      Shirt::Blue,
    ]
  };

  let user_preference = [
    Some(Shirt::Red),
    None
  ];

  let giveaway = [
    store.giveaway(user_preference[0]),
    store.giveaway(user_preference[1]),
  ];

  println!("\n");
  println!("The user with preference {:?} gets {:?}", user_preference[0], giveaway[0]);
  println!("The user with preference {:?} gets {:?}", user_preference[1], giveaway[1]);
}