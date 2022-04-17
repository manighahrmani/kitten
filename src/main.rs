use kitten::{file_helper, string_helper};
use std::env;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let passed_args: env::Args = env::args();
  let filenames: Vec<String>;
  match arguments::parse(passed_args) {
    Ok(args) => {
      filenames = args.orphans;
    }
    Err(error) => {
      panic!("Error while parsing your input: {}", error);
    }
  }

  // TODO: Add a check for help function

  // let output: String =
  //   filenames
  //     .iter()
  //     .enumerate()
  //     .fold(String::new(), |accumulator, (index, filename)| {
  //         println!(
  //           "{} file is being processed: {}",
  //           string_helper::as_ordinal(index as u32),
  //           filename
  //         );
  //       let file_content: String = file_helper::file_content(filename).unwrap_or_else(|error| {
  //         panic!(
  //           "Error while reading file {}: {}",
  //           filename,
  //           error
  //         );
  //       });
  //       accumulator + &file_content
  //     });

  let mut output = String::new();

  for index in 0..filenames.len() {
    let filename: &str = filenames[index].as_str();

    println!(
      "{} file is being processed: {}",
      string_helper::as_ordinal((index + 1) as u32),
      filename
    );

    match file_helper::file_content(filename) {
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
