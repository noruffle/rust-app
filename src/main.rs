use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

  // 1. Start game
  println!("Guess the number!");
  
  // 2. Creating Secret number
  // 2.1. Library "rand" for generating a number.
  // 2.1.1. "thread_rng()" function that gives us the particular random number generator.
  // 2.1.2. "gen_range(1..=100)" method of "Rng" trait that takes a range expression 
  //        as an argument and generates a random number in the range. The kind of 
  //        range expression we're using here takes form "start..=end".
  let secret_number = rand::thread_rng().gen_range(1..=100);

  // 2.1. Output of Secret number
  println!("Secret number is: {secret_number}");

  // 3. Creating the loop
  loop {

    // 4. Creating "guess" variable, it consider user input String
    // 4.1. "let" is immutable by default, to make mutable -> add "mut"
    let mut guess = String::new();
  
    // 5. "std" is library; "io" is module; 
    // 5.1. "stdin()" is fn, which allow us to handle user input data.
    // 5.1.1. "read_line(&mut guess)" is the method, input handler,
    //        that allow us to get input from user. 
    // 5.1.2. "&mut guess" as the argument, that be readed.
    //        And "&" is the reference.
    // 5.1.3. "expect(msg: String)" is the methid, error handler,
    //        that allow us to throw an Error with String msg.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
  
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
  
    println!("You guessed: {guess}");
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
    }
  }
}