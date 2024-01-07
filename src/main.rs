mod guess; 
use guess::{
  Guess, TraitGuess
};

mod utils; 
use utils::{
  Utils, TraitUtils
};

mod figures; 
use figures::{
  Circle, Rectangle, Area
};

mod articles; 
use articles::{
  Summary, Comment, Anime
};

mod shop; 
use shop::{
  Inventory, User, Color
};

mod cline; 
use cline::{smth, search};

mod tests;


fn main() {
  let vec: Vec<i32> = vec![10000, 2, 99, 4, 100, 55];
  let switch: i32 = 7;

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
      smth()
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
    },
    7 => {
      let (query, cx) = ("safe", "fast");
      search(query, cx);
    },
    _ => println!("Try again")

  }
}