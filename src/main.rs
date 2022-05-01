use kitten::file_helper;
use std::{env, process};

fn main() {
  let passed_args: env::Args = env::args();
  let filenames: Vec<String>;
  match arguments::parse(passed_args) {
    Ok(args) => {
      filenames = args.orphans;
    }
    Err(error) => {
      // TODO: Need to explain this
      eprintln!("Error while parsing your input: {}", error);
      process::exit(1);
    }
  }

  // TODO: Add a check for help function

  let output: String =
    filenames
      .iter()
      .enumerate()
      .fold(String::new(), |accumulator, (_index, filename)| {
        // println!(
        //   "{} file is being processed: {}",
        //   string_helper::as_ordinal(index as u32),
        //   filename
        // );
        let file_content: String = file_helper::file_content(filename).unwrap_or_else(|error| {
          // TODO: Need to explain this
          eprintln!("Error while reading file {}: {}", filename, error);
          process::exit(1);
        });
        accumulator + &file_content
      });

  print!("{output}");
}
