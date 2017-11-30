// reads numbers from stdin, prints sum
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let reader = BufReader::new(stdin);
  let mut array : Vec<u32> = reader.lines().map(|line| line.unwrap().parse().expect("Wanted a number")).collect();
  
  array.sort();

  for number in array {
    println!("{}", number);
  }
}
