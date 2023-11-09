mod figures;
mod utils;
mod guess;
mod articles;
mod tests;
mod cline;
mod shop;

use guess::{Guess, TraitGuess};
use utils::{Utils, TraitUtils};
use figures::{Circle, Rectangle, Area};
use articles::{Summary, Comment};
use cline::*;
use shop::{struct_::Inventory, enum_::Color};

use crate::{articles::Anime, shop::struct_::User};

fn main() {
  let vec: Vec<i32> = vec![10000, 2, 99, 4, 100, 55];
  let switch: i32 = 6;

  match switch {
    1 => Guess{status: true}.check(),
    2 => Utils::multiplying(vec),
    3 => {

      let cir: Circle<'_> = Circle::new(25.0);

      let rec: Rectangle<'_> = Rectangle::new(Some((200, 200)));
      
      cir.area();
      rec.area();

    },
    4 => {
      let comment: Comment = Comment {
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
      search_for_env()
    },
    6 => {
      let store: Inventory = Inventory {
        shirts: vec![Color::Black, Color::Black, Color::White],
      };


      let ruffle: User = User {
        name: "Ruffle"
      };
      let ruffle_preference: Option<Color> = Some(Color::Black);
      let giveaway_ruffle: Color = store.giveaway(ruffle_preference);

      println!("The user {:?} with preference {:?} gets {:?}", ruffle.name, ruffle_preference, giveaway_ruffle);
    }
    _ => println!("Try again")

  }
}