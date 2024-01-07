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