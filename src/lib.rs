pub mod string_helper {
  /// Retruns the ordinal number (e.g. 1st) given a positive integer
  ///
  /// # Arguments
  ///
  /// * `number` - The integer to convert to an ordinal number
  ///
  /// # Examples
  ///
  /// ```
  /// use kitten::string_helper::as_ordinal;
  /// let arg = 1;
  /// let answer = as_ordinal(arg);
  ///
  /// assert_eq!(String::from("1st"), answer);
  /// ```
  pub fn as_ordinal(number: u32) -> String {
    number.to_string()
      + match number % 10 {
        1 if number % 100 != 11 => "st",
        2 if number % 100 != 12 => "nd",
        3 if number % 100 != 13 => "rd",
        _ => "th",
      }
  }

  /// Retruns the first word of a given string
  ///
  /// # Arguments
  ///
  /// * `text` - The string to get the first word from
  ///
  /// # Examples
  ///
  /// ```
  /// use kitten::string_helper::first_word;
  /// let arg = String::from("23 files ");
  /// let answer = first_word(arg);
  ///
  /// let expected_answer = String::from("23");
  ///
  /// assert_eq!(expected_answer, answer);
  /// ```
  pub fn first_word(text: String) -> String {
    let mut words = text.split_whitespace();
    let first_word = words.next();
    match first_word {
      Some(word) => word.to_string(),
      None => String::new(),
    }
  }
}

#[cfg(test)]
mod string_helper_tests {

  mod as_ordinal_tests {
    use crate::string_helper;

    #[test]
    fn as_ordinal_th() {
      let cases: Vec<(u32, &str)> = vec![
        (10, "10th"),
        (111, "111th"),
        (0, "0th"),
        (999, "999th"),
        (250, "250th"),
      ];
      for case in cases {
        as_ordinal_test(case.0, String::from(case.1));
      }
    }
    #[test]
    fn as_ordinal_not_th() {
      let cases: Vec<(u32, &str)> = vec![
        (1, "1st"),
        (2, "2nd"),
        (3, "3rd"),
        (101, "101st"),
        (102, "102nd"),
        (103, "103rd"),
        (501, "501st"),
        (502, "502nd"),
        (503, "503rd"),
        (10901, "10901st"),
        (10902, "10902nd"),
        (10903, "10903rd"),
      ];
      for case in cases {
        as_ordinal_test(case.0, String::from(case.1));
      }
    }

    fn as_ordinal_test(number: u32, expected_answer: String) {
      let answer = string_helper::as_ordinal(number);
      assert_eq!(expected_answer, answer);
    }
  }

  mod first_word_tests {
    use crate::string_helper;

    #[test]
    fn test_first_word_works() {
      let cases: Vec<(&str, &str)> = vec![
        ("🐭🦊 🐻🦁 🐻‍❄️🐼🐶", "🐭🦊"),
        ("Lorem ipsum dolor sit amet, ", "Lorem"),
        (
          "Lorem 
ipsum dolor sit amet, ",
          "Lorem",
        ),
        ("", ""),
      ];
      for case in cases {
        test_first_word(String::from(case.0), String::from(case.1));
      }
    }

    fn test_first_word(text: String, expected_answer: String) {
      let answer = string_helper::first_word(text);
      assert_eq!(expected_answer, answer);
    }
  }
}

pub mod file_helper {

  use std::fs;

  /// Retruns the a result that is either Ok(conentet)
  /// where content is the file's conent as a String
  /// or Err(error) where error is the error message as a String
  ///
  /// # Arguments
  ///
  /// * `filename` - The string slice of the path to the file
  ///
  /// # Errors
  ///
  /// * If the file does not exist - "`filename` does not exist"
  /// * If the file is not readable - "`filename` is not readable"
  /// * For all other errors - "Unknown error opening `filename`"
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use kitten::file_helper::file_content;
  ///
  /// fn main() {
  ///    let mut output = file_content("foo.txt");
  ///    match output {
  ///     Ok(content) => println!("File's conentent is: {}", content),
  ///     Err(error) => println!("Error opening file: {}", error),
  ///     };
  /// }
  /// ```
  pub fn file_content(filename: &str) -> Result<String, String> {
    match fs::read_to_string(filename) {
      Ok(content) => Ok(content),
      Err(error) if format!("{}", error).contains("No such file") => {
        Err(format!("{} does not exist", filename))
      }
      Err(error) if format!("{}", error).contains("Permission denied") => {
        Err(format!("{} is not readable", filename))
      }
      Err(_) => Err(format!("Unknown error opening {}", filename)),
    }
  }
}

#[cfg(test)]
mod file_helper_tests {
  use super::file_helper;

  #[test]
  fn file_content_file_not_exists() {
    let filename = "imaginary_dir/imaginary_file.txt"; // does not exist
    assert!(file_helper::file_content(filename).is_err());
    assert_eq!(
      file_helper::file_content(filename).unwrap_err(),
      "imaginary_dir/imaginary_file.txt does not exist"
    );
  }

  #[test]
  fn file_content_file_not_readable() {
    let filename = "locked.txt"; // is a file that does not have any permissions
    assert!(file_helper::file_content(filename).is_err());
    assert_eq!(
      file_helper::file_content(filename).unwrap_err(),
      "locked.txt is not readable"
    );
  }

  #[test]
  fn file_content_file_exists() {
    let filename = "text.txt"; // is a file with content below
    let content = String::from(
      "I'm closer to the Golden Dawn
Immersed in Crowley's uniform of imagery
I'm living in a silent film
Portraying Himmler's sacred realm of dream reality
",
    );
    assert_eq!(content, file_helper::file_content(filename).unwrap());
  }
}

pub mod option_helper {
  pub const MANUAL: &str = "NAME:
  kitten - A mini version of the cat command

SYNOPSIS:
  kitten [OPTION]... [FILE]...

DESCRIPTION:
  Concatenates FILE(s) to standard output.
  With no FILE returns an empty string.

  --h, --help
      display this help and exit
  
EXAMPLE:
  kitten foo.txt bar.txt
    Outputs the contents of foo.txt and then bar.txt.
";

  /// Accepts the options taken from the command line
  /// and returns an Option that is either Some(manual)
  /// where manual is the help manual as a String
  /// or None if help option is not set.
  ///
  /// # Arguments
  ///
  /// * `options` - The options taken as arguments from the command line
  ///
  /// # Examples
  ///
  /// ```
  /// use kitten::option_helper::handle_options;
  /// use arguments::Arguments;
  ///
  /// let arguments = std::env::args(); // kitten --help or kitten --h
  /// let arguments = arguments::parse(arguments).unwrap();
  ///
  /// let mut options = arguments.options;
  /// options.set("help", true);
  ///
  /// let output: String = handle_options(options).unwrap();
  /// let expected_manual: &str = "NAME:
  ///   kitten - A mini version of the cat command
  ///
  /// SYNOPSIS:
  ///   kitten [OPTION]... [FILE]...
  ///
  /// DESCRIPTION:
  ///   Concatenates FILE(s) to standard output.
  ///   With no FILE returns an empty string.
  ///
  ///   --h, --help
  ///       display this help and exit
  ///  
  /// EXAMPLE:
  ///   kitten foo.txt bar.txt
  ///     Outputs the contents of foo.txt and then bar.txt.
  /// ";
  ///
  /// assert_eq!(String::from(expected_manual), output);
  /// ```
  pub fn handle_options(options: arguments::Options) -> Option<String> {
    if options.has("help") || options.has("h") {
      Some(MANUAL.to_string())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod option_helper_tests {
  use super::option_helper;

  #[test]
  fn handle_options_help() {
    let mut options_one: arguments::Options = arguments::Options::new();
    options_one.set("help", true);
    assert_eq!(
      option_helper::handle_options(options_one),
      Some(option_helper::MANUAL.to_string())
    );

    let mut options_two: arguments::Options = arguments::Options::new();
    options_two.set("h", true);
    assert_eq!(
      option_helper::handle_options(options_two),
      Some(option_helper::MANUAL.to_string())
    );
  }

  #[test]
  fn handle_options_other() {
    let mut options: arguments::Options = arguments::Options::new();
    options.set("other", true);
    assert_eq!(option_helper::handle_options(options), None);
  }
}
