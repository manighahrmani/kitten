use kitten::{file_helper, option_helper};
use std::{env, process};

fn main() {
  let passed_args: env::Args = env::args();
  let filenames: Vec<String>;
  let options: arguments::Options;

  match arguments::parse(passed_args) {
    Ok(args) => {
      filenames = args.orphans;
      options = args.options;
    }
    Err(error) => {
      eprintln!("Error while parsing your input: {}", error);
      process::exit(1);
    }
  }

  match option_helper::handle_options(options) {
    Some(manual) => println!("{}", manual),
    None => {
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
            let file_content: String =
              file_helper::file_content(filename).unwrap_or_else(|error| {
                eprintln!("Error while reading file {}: {}", filename, error);
                process::exit(1);
              });
            accumulator + &file_content
          });

      print!("{output}");
    }
  }
}
