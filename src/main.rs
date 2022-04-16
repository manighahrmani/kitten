use kitten::{file_helper, string_helper};
use std::env;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let passed_args: env::Args = env::args();
  let filenames: Vec<String> = arguments::parse(passed_args).unwrap().orphans;

  // TODO: Add a check for help function

  let mut output = String::new();

  for index_of_filename in 0..filenames.len() {
    let filename: &str = filenames.get(index_of_filename).unwrap();

    println!(
      "{} file is being processed: {}",
      string_helper::as_ordinal(index_of_filename as u32),
      filename
    );

    let file_content_result: Result<String, String> = file_helper::file_content(filename);
    match file_content_result {
      Ok(file_content) => {
        output += &file_content;
      }
      Err(error) => {
        panic!("Error while reading file {}: {}", filename, error);
      }
    }
  }

  println!("Here is the output:\n{}", output);

  println!("Bye from {}!", KITTEN);
}
