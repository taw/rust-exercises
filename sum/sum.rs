// reads numbers from stdin, prints sum
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let mut sum = 0;
  let stdin = io::stdin();

  let reader = BufReader::new(stdin);
  
  for line in reader.lines() {
    let number : i32 = line.unwrap().parse().expect("Wanted a number");
    sum += number;
  }
  println!("{}", sum);
}
