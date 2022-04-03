#![allow(unused)]

const MAX_U32: u32 = std::u32::MAX;
const MAX_ERR: &str = "Error parsing your input: At most 4294967295 files are allowed";
const MIN_ERR: &str = "Error parsing your input: At least 1 file is required";
const OTHER_ERR: &str = "Error parsing your input: Please provide a valid number";

pub fn parse_number_of_files(input: &str) -> Result<u32, &'static str> {
  match input.parse::<isize>() {
    Ok(num) if num > MAX_U32 as isize => Err(MAX_ERR),
    Ok(num) if num <= 0 => Err(MIN_ERR),
    Ok(num) => Ok(num as u32),
    Err(_) => Err(OTHER_ERR),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_number_of_files_ok() {
    assert_eq!(parse_number_of_files("1").unwrap(), 1);
    assert_eq!(
      parse_number_of_files(&MAX_U32.to_string()).unwrap(),
      MAX_U32
    );
  }

  #[test]
  fn parse_number_of_files_out_of_bound() {
    assert_eq!(parse_number_of_files("0").unwrap_err(), MIN_ERR);
    assert_eq!(parse_number_of_files("-1").unwrap_err(), MIN_ERR);
    let max_plus_one: u64 = MAX_U32 as u64 + 1;
    assert_eq!(
      parse_number_of_files(&max_plus_one.to_string()).unwrap_err(),
      MAX_ERR
    );
  }

  #[test]
  fn parse_number_of_files_invalid() {
    assert_eq!(parse_number_of_files("--").unwrap_err(), OTHER_ERR);
    assert_eq!(parse_number_of_files(" 200").unwrap_err(), OTHER_ERR);
  }
}
