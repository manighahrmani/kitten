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
    println!("{}", get_filename_query(i, number_of_files));

    // need to ask for input here
  }

  println!("Bye from {}!", KITTEN);
}

fn ordinal_value(number: u32) -> String {
  number.to_string()
    + match number % 10 {
      1 if number != 11 => "st",
      2 if number != 12 => "nd",
      3 if number != 13 => "rd",
      _ => "th",
    }
}

fn singular_or_plural(number: u32) -> &'static str {
  if number == 1 {
    "file"
  } else {
    "files"
  }
}

fn get_filename_query(number_of_file: u32, total_number_of_files: u32) -> String {
  let query = match total_number_of_files - number_of_file {
    0 => String::from("Please enter the name/path to the last file:"),
    number_of_files_left => {
      format!(
        "{} more filename{} to go.\nPlease enter the name/path to the {} file: ",
        number_of_files_left,
        singular_or_plural(number_of_file),
        ordinal_value(number_of_file)
      )
    }
  };
  query
}

fn _get_first_word(input: String) -> String {
  let mut first_word = String::new();
  for c in input.chars() {
    if c == ' ' {
      break;
    }
    first_word.push(c);
  }
  first_word
}

fn _keep_only_first_word(input: String) -> String {
  let mut words = input.split_whitespace();
  let first_word = words.next().unwrap();
  first_word.to_string()
}
