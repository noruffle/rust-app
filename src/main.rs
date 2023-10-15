mod figures;
mod utils;
mod guess;
mod articles;

use guess::{Guess, TraitGuess};
use utils::{Utils, TraitUtils};
use figures::{Circle, Rectangle, Drawable};
use articles::{Summary, Comment};

use crate::articles::Anime;

fn main() {
  let vec = vec![10000, 2, 99, 4, 100, 55];
  let switch = 5;

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
    },
    4 => {
      let comment = Comment {
        username: String::from("Ruffle"),
        content: String::from("Hellow, world!"),
        reply: false,
        repost: false,
      };

      println!("1 new comment: '{}'", comment.summarize());

      let anime = Anime {
        title: String::from("Kakkou no Iinazuke"),
        genres: String::from("Comedy, Domance"),
        episodes: 24,
        status: "Finished".to_string(),
      };

      println!("New anime started! {}", anime.summarize());


    },
    5 => {

    }
    _ => println!("Try again")

  }
}