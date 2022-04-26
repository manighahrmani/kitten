use kitten::file_helper;
use std::env;

fn main() {
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
          panic!("Error while reading file {}: {}", filename, error);
        });
        accumulator + &file_content
      });

  print!("{output}");
}
