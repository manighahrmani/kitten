use std::io;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32 = 1;

  println!("How many files would you like to open?");

  let mut input: String = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Error while reading your input!");

  input = input.trim().to_string();
  println!("Your input was: ({})", input);

  println!("We need to open {} files.", number_of_files);

  println!("Bye from {}!", KITTEN);
}
