use std::error::Error;

pub trait TraitUtils {
  fn multiplying(vector: Vec<i32>);
  fn show_in_vector(vector: Vec<i32>);
  fn searching_vector(vector: &Vec<i32>, index: usize) -> Result<(), Box<dyn Error>>;
  fn converts_into_bytes(string: &str);
  fn open_txt(path: &str) -> Result<(), Box<dyn Error>>;
}