pub mod string_helper {
  /// Retruns the ordinal number given a positive integer
  ///
  /// # Arguments
  ///
  /// * `number` - The number to convert to an ordinal number
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
  /// * `text` - The text to get the first word from
  ///
  /// # Examples
  ///
  /// ```
  /// use kitten::string_helper::first_word;
  /// let arg = "23 files ";
  /// let answer = first_word(arg);
  ///
  /// assert_eq!(23, answer);
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
