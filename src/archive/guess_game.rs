use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn guess_game() {

  // 1. Start game
  println!("Guess the number!");
  
  // 2. Creating Secret number
  // 2.1. Library "rand" for generating a number.
  // 2.1.1. "thread_rng()" function that gives us the particular random number generator.
  // 2.1.2. "gen_range(1..=100)" method of "Rng" trait that takes a range expression 
  //        as an argument and generates a random number in the range. The kind of 
  //        range expression we're using here takes form "start..=end".
  let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

  // 2.1. Output of Secret number
  // println!("Secret number is: {secret_number}");

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
  
    // 7. Using Shadowing, that lets us reuse the guess variable name rather than
    //    forcing us to create two unique varibales, such as "guess_str" and "guess", for example.
    // 7.1. For avoiding mismatching types, we adding unsigned 32-bit number type.
    //      i32 by default, signed.
    // 7.2. New expression "guess.trim().parse()".
    // 7.2.1. The "guess" in the expression refers to the original "guess" variable,
    //        that contained the input as a String.
    // 7.2.2. The "trim()" is methid on a String instance will eliminate any whitespace
    //        at the beggining and end, which we must do to be able to compare the string to the u32,
    //        which can only contain numerical data.
    // 7.2.3. The "parse()" us the method on String converts a String to another type.
    //        Here, we use it to conver from a String to a Number.
    // 7.2.4. The "expect(msg)" is an Error handler.
    // 7.3. We need to tell Rust the exact number type we want by using "let guess: u32".
    // 7.4. If inputed number = number, if "afghsdh" = ignore (continue).
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    
    println!("You guessed: {guess}");
  
    // 6. The "match" is like "switch..case"
    // 6.1. The "cmp()" is the method, that compares two values and can be 
    //      called on anything that cab be compared. It takes a reference 
    //      to whatever you want to compare with: here it's comparing
    //      "guess" to "secret_number". Then it returns a variant of the "Ordering" enum.
    // 6.2. The "Ordering" is an enum, it has the variants: Less, Greater, Equal.
    //      There are the three outcomes that are possible when you compare two values.
    match guess.cmp(&secret_number) {
      // guess < secret
      Ordering::Less => println!("Too small!"),
      // guess > secret
      Ordering::Greater => println!("Too big!"),
      // guess = secret
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}