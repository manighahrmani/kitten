use std::io;

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi from {}!", KITTEN);

  let number_of_files: u32 = 1;

  println!("How many files would you like to open?");

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes read", n);
      println!("Your input was: ({})", input);
    }
    Err(error) => println!("Error while reading your input: {}", error),
  }

  //  let mut input = String::new();
  //  if let Err(e) = io::stdin().read_line(&mut input) {
  //   println!("Error while reading your input: {}", e);
  // } else {
  //   println!("Your input was: ({})", input);
  // }

  // let mut input = String::new();
  //  io::stdin()
  //  .read_line(&mut input)
  //  .expect("Error while reading your input!");
  // println!("Your input was: ({})", input);

  println!("We need to open {} files.", number_of_files);

  println!("Bye from {}!", KITTEN);
}
