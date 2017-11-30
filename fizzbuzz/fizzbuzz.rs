fn fizzbuzz(i : u32) -> String {
  if i % 15 == 0 {
    return "FizzBuzz".to_string();
  } else if i % 3 == 0 {
    return "Fizz".to_string();
  } else if i % 5 == 0 {
    return "Buzz".to_string();
  } else {
    return i.to_string();
  }
  
}

fn main() {
  let mut i = 1;
  while i <= 100 {
    println!("{}", fizzbuzz(i));
    i += 1;
  }
}
