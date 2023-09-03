#[derive(Debug)]
enum Class {
  Warrior,
}
#[derive(Debug)]
struct User {
  name: String,
  age: u32,
  class: Class,
  level: u32,

  health: u32,
  mana: u32,
}
fn main() {
  let mut users = vec![
    User {
      name: String::from("Ruffle"),
      age: 24,
      class: Class::Warrior,
      level: 70, 
      health: 0, 
      mana: 0 
    }
  ];

  for user in users.iter_mut() {
    if user.level > 0 {
      user.health = user.health + user.level * 5;
      user.mana = user.mana + user.level * 2;
    }
  }
  
  println!("{:?}", users)
}