mod figures;
mod utils;
mod guess;

use guess::{Guess, TraitGuess};
use utils::{Utils, TraitUtils};
use figures::{Circle, Rectangle, Drawable};

fn main() {
  let vec = vec![1, 2, 3, 4, 5];
  let switch = 0;

  match switch {
    1 => Guess{status: true}.check(),
    2 => Utils::multiplying(vec),
    3 => {
      let rec = Rectangle {
        height: 100,
        width: 200,
        dimension: None,
      };

      let cir = Circle {
        radius: 25.0,
      };

      rec.area();
      rec.draw();

      cir.area();
      cir.draw();
    }
    _ => println!("Try again")

  }
}