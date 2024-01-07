mod guess; 
mod utils; 
mod figures; 
mod articles; 
mod shop; 
mod cli; 
mod mods;

use mods::*;
use std::*;

fn main() {
  let vec: Vec<i32> = vec![10000, 2, 99, 4, 100, 55];
  let switch: i32 = 7;

  match switch {
    1 => {
      Guess{status: true}.check()
    },
    2 => {
      Utils::multiplying(vec)
    },
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
      smth();

      let (query, cx) = ("safe", "fast");
      search(query, cx);

      search_case_insensitive(query, cx);
    },
    6 => {
      store();
    },
    7 => {
      println!("Before defining closure: {:?}", vec);

      thread::spawn( move || println!("From thread: {:?}", vec ))
        .join()
        .unwrap();
    },
    _ => println!("Try again")

  }
}