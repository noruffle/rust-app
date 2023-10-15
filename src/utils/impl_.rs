use std::error::Error;
use std::fs::File;
use std::io::Read;

use super::trait_::TraitUtils;
use super::struct_::Utils;

impl TraitUtils for Utils {
/*   fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
} */


  fn converts_into_bytes(string: &str) {
    for bytes in string.bytes() {
      println!("{bytes}");
    }
  }

  fn multiplying(mut vector: Vec<i32>) {
    for value in &mut vector {
      *value *= 2;
    }
    self::Utils::show_in_vector(vector)
  }

  fn open_txt(path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut data = vec![];

    let _ = file.read_to_end(&mut data);

    Ok(())
  }

  fn searching_vector(vector: &Vec<i32>, index: usize) -> Result<(), Box<dyn Error>> {
    let third: &i32 = &vector[6];
    println!("The third element is {third}");

    let third: Option<&i32> = vector.get(index);
    match third {
      Some(third) => println!("The third element is {third}"),
      None => println!("No third")
    }

    Ok(())
  }

  fn show_in_vector(vector: Vec<i32>) {
    for value in &vector {
      println!("{value}");
    }
  }
}