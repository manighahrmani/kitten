#![allow(unused)]

pub fn parse_number_of_files(input: &str) -> Result<u32, &'static str> {
  match input.parse::<isize>() {
    Ok(num) => {
      if num <= 0 {
        Err("Error while parsing your input: You need to provide at least 1 filename")
      } else if num > 4_294_967_295 {
        Err("Error while parsing your input: You the maximum number of files is 4294967295")
      } else {
        Ok(num as u32)
      }
    }
    Err(_) => Err("Error while parsing your input: Input cannot be converted into a number (i64)"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_number_of_files_ok() {
    assert_eq!(parse_number_of_files("1").unwrap(), 1);
    assert_eq!(parse_number_of_files("4294967295").unwrap(), 4294967295);
  }

  #[test]
  fn parse_number_of_files_out_of_bound() {
    assert_eq!(
      parse_number_of_files("0").unwrap_err(),
      "Error while parsing your input: You need to provide at least 1 filename"
    );
    assert_eq!(
      parse_number_of_files("-1").unwrap_err(),
      "Error while parsing your input: You need to provide at least 1 filename"
    );
    assert_eq!(
      parse_number_of_files("4294967296").unwrap_err(),
      "Error while parsing your input: You the maximum number of files is 4294967295"
    );
  }

  #[test]
  fn parse_number_of_files_invalid() {
    assert_eq!(
      parse_number_of_files("--").unwrap_err(),
      "Error while parsing your input: Input cannot be converted into a number (i64)"
    );
    assert_eq!(
      parse_number_of_files(" 200").unwrap_err(),
      "Error while parsing your input: Input cannot be converted into a number (i64)"
    );
  }
}
