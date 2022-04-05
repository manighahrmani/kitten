#![allow(unused)]

const U32_MAX: u32 = std::u32::MAX;
const PARSE_ERR_MAX: &str = "Error parsing your input: At most 4294967295 files are allowed";
const PARSE_ERR_MIN: &str = "Error parsing your input: At least 1 file is required";
const PARSE_ERR_OTHER: &str = "Error parsing your input: Please provide a valid number";

pub fn parse_number_of_files(input: &str) -> Result<u32, &'static str> {
  match input.parse::<isize>() {
    Ok(num) if num > U32_MAX as isize => Err(PARSE_ERR_MAX),
    Ok(num) if num <= 0 => Err(PARSE_ERR_MIN),
    Ok(num) => Ok(num as u32),
    Err(_) => Err(PARSE_ERR_OTHER),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_number_of_files_ok() {
    assert_eq!(parse_number_of_files("1").unwrap(), 1);
    assert_eq!(
      parse_number_of_files(&U32_MAX.to_string()).unwrap(),
      U32_MAX
    );
  }

  #[test]
  fn parse_number_of_files_out_of_bound() {
    assert_eq!(parse_number_of_files("0").unwrap_err(), PARSE_ERR_MAX);
    assert_eq!(parse_number_of_files("-1").unwrap_err(), PARSE_ERR_MIN);
    let max_plus_one: u64 = U32_MAX as u64 + 1;
    assert_eq!(
      parse_number_of_files(&max_plus_one.to_string()).unwrap_err(),
      PARSE_ERR_MAX
    );
  }

  #[test]
  fn parse_number_of_files_invalid() {
    assert_eq!(parse_number_of_files("--").unwrap_err(), PARSE_ERR_OTHER);
    assert_eq!(parse_number_of_files(" 200").unwrap_err(), PARSE_ERR_OTHER);
  }
}
