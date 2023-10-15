/* pub enum AirStatus {
  Finished,
  Ongoing,
}

pub enum Genres {
  Action,
  Adventure,
  Comedy,
  Drama,
  Fantasy,
  Horror,
  Mystery,
  Romance,
  SciFi,
  SliceOfLife,
  Sports,
  Supernatural,
  Suspense,
} */

pub struct Anime {
  pub title: String,
  pub episodes: i32,
  pub status: String,
  pub genres: String
}

pub struct Comment {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub repost: bool,
}