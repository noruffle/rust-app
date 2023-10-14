use std::error::Error;

pub trait TraitGuess {
  fn check(&self);
  fn start(&self) -> Result<(), Box<dyn Error>>;
}