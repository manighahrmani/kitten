use std::io;

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

mod string_helper {
  /// Retruns the ordinal number given a positive integer.
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = 23;
  /// let answer = kitten::string_helper::as_ordinal(arg);
  ///
  /// assert_eq!(String::from("23rd"), answer);
  /// ```
  fn as_ordinal(number: u32) -> String {
    number.to_string()
      + match number % 10 {
        1 if number % 100 != 11 => "st",
        2 if number % 100 != 12 => "nd",
        3 if number % 100 != 13 => "rd",
        _ => "th",
      }
  }

  /// Retruns the first word of a given string.
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = String::from("23 files ");
  /// let answer = kitten::string_helper::first_word(arg);
  ///
  /// assert_eq!(String::from("23"), answer);
  /// ```
  fn first_word(input: String) -> String {
    let mut words = input.split_whitespace();
    let first_word = words.next();
    match first_word {
      Some(word) => word.to_string(),
      None => String::new(),
    }
  }
}
