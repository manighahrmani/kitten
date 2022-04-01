use std::io;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32;

  println!("How many files would you like to open?");

  let mut input: String = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Error while reading your input!"); 
  // on error, the program will panic and crash

  input = input.trim().to_string();
  println!("Your input was: ({})", input);

  // on parsin errors, no error message will be printed
  number_of_files = input.parse().unwrap_or(1);
  // match input.parse::<u32>() {
  //   Ok(num) => {
  //     number_of_files = num;
  //   }
  //   Err(e) => {
  //     println!("Error on parsing input: {}", e);
  //   }
  // }
  println!("We need to open {} files.", number_of_files);

  println!("Bye from {}!", KITTEN);
}
