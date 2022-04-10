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
