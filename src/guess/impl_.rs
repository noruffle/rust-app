use std::{cmp::Ordering, io};
use rand::Rng;

use super::trait_::TraitGuess;
use super::struct_::Guess;

impl TraitGuess for Guess {
  fn check(&self) {
    if self.status == true {
      let _ = self.start();
    }
  }

  fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
  
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };
    
      println!("You guessed: {guess}");
  
      match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        },
      }
    }

    Ok(())
  }
}