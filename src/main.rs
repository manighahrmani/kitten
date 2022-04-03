use std::io;

mod lib;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32;

  println!("How many files would you like to open?");

  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      input.pop();
      println!("Your input was: ({})", input);
    }
    Err(error) => {
      panic!("Error while reading your input: {}", error);
    }
  }

  number_of_files = kitten::parse_number_of_files(&input).unwrap();

  for i in 1..number_of_files {
    println!("Please enter the name/path to a file {}: ", i);
    // need to ask for an input here
  }

  println!("Bye from {}!", KITTEN);
}
