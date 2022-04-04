use std::io;

mod lib;

use kitten::parse_number_of_files;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32;

  println!("How many files would you like to open?");

  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      input.pop();
    }
    Err(error) => {
      panic!("Error while reading your input: {}", error);
    }
  }

  number_of_files = parse_number_of_files(&input).unwrap();

  for i in 1..(number_of_files + 1) {
    let ordinal = match i {
      i if i % 10 == 1 && i != 11 => "st",
      i if i % 10 == 2 && i != 12 => "nd",
      i if i % 10 == 3 && i != 13 => "rd",
      _ => "th",
    };

    match number_of_files - i {
      0 => println!("Please enter the name/path to the last file:"),
      n => {
        let singular_or_plural = if n == 1 { "filename" } else { "filenames" };
        println!(
          "{} more {} to go.\nPlease enter the name/path to the {}{} file: ",
          number_of_files - i,
          singular_or_plural,
          i,
          ordinal
        );
      }
    };

    // need to ask for input here
  }

  println!("Bye from {}!", KITTEN);
}
