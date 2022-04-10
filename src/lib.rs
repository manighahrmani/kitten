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
  use super::*;

  #[test]
  fn test_as_ordinal_th() {
    let cases: Vec<(u32, &str)> = vec![
      (10, "10th"),
      (111, "111th"),
      (0, "0th"),
      (999, "999th"),
      (250, "250th"),
    ];
    for case in cases {
      test_as_ordinal(case.0, String::from(case.1));
    }
  }
  #[test]
  fn test_as_ordinal_not_th() {
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
      test_as_ordinal(case.0, String::from(case.1));
    }
  }

  fn test_as_ordinal(number: u32, expected_answer: String) {
    let answer = string_helper::as_ordinal(number);
    assert_eq!(expected_answer, answer);
  }

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
