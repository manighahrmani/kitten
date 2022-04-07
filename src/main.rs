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
      input = first_word(input);
    }
    Err(error) => {
      panic!("Error while reading your input: {}", error);
    }
  }

  number_of_files = parse_number_of_files(&input).unwrap();

  for number_of_file in 1..(number_of_files + 1) {
    println!(
      "[{}/{}] Please enter the name/path to the {} file:",
      number_of_file,
      number_of_files,
      as_ordinal(number_of_file)
    );

    // need to ask for input here
  }

  println!("Bye from {}!", KITTEN);
}

fn as_ordinal(number: u32) -> String {
  number.to_string()
    + match number % 10 {
      1 if number % 100 != 11 => "st",
      2 if number % 100 != 12 => "nd",
      3 if number % 100 != 13 => "rd",
      _ => "th",
    }
}

fn first_word(input: String) -> String {
  let mut words = input.split_whitespace();
  let first_word = words.next();
  match first_word {
    Some(word) => word.to_string(),
    None => String::new(),
  }
}

// An alternative to the above function
// fn _first_word(input: String) -> String {
//   let mut first_word = String::new();
//   for c in input.chars() {
//     if c == ' ' || c == '\n' {
//       break;
//     }
//     first_word.push(c);
//   }
//   first_word
// }
