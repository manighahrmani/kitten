use kitten::{file_helper, string_helper};
use std::io;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32;

  println!("How many files would you like to open?");

  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      input = string_helper::first_word(input);
    }
    Err(error) => {
      panic!("Error while reading your input: {}", error);
    }
  }

  match input.parse::<u32>() {
    Ok(0) => {
      panic!("Error while parsing your input: You need to provide at least 1 filename");
    }
    Ok(num) => {
      number_of_files = num;
    }
    Err(error) => {
      panic!("Error while parsing your input: {}", error);
    }
  }

  let mut output = String::new();

  for number_of_file in 1..(number_of_files + 1) {
    println!(
      "[{}/{}] Please enter the name/path to the {} file:",
      number_of_file,
      number_of_files,
      string_helper::as_ordinal(number_of_file)
    );

    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        input = string_helper::first_word(input);
        println!("Opening file: {}", input); // for debugging
      }
      Err(error) => {
        panic!("Error while reading your input: {}", error);
      }
    }
    //--snip--
    // TODO: remove this
    // let filename = input.clone();
    output = output + &file_helper::file_content(&input).unwrap();
  }

  println!("Here is the output:\n{}", output);

  println!("Bye from {}!", KITTEN);
}
