use std::io::prelude::*;
use std::fs::File;

fn get_corpus() -> String {
  let mut file = match File::open("./corpus.txt") {
      Ok(file) => file,
      Err(..)  => panic!("can't open corpus file"),
  };
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn main() {
  println!("{}", get_corpus());
}
