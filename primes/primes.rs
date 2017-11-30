fn is_prime (n : u32) -> bool {
  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      return false;
    }
    i += 1;
  }
  return true;
}

fn main() {
  let mut i = 2;

  while i <= 1000 {
    if is_prime(i) {
      println!("{}", i);
    }
    i += 1
  }
}
