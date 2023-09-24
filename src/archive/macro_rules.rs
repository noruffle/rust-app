use std::io;

macro_rules! greet {
    ($name:expr) => {
        let name = $name;
        println!("Hello, {}", name);
    };
}

fn main() {
  println!("Insert the name.");
  
  let mut name = String::new();

  io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line.");

  greet!(name);
}