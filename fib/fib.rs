use std::env;

fn fib(n : u32) -> u32 {
  if n <= 1 {
    return n;
  } else {
    return fib(n-1) + fib(n-2);
  }
}

// Expect fib(10) = 55
fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: ./fib N");
  } else {
    let n = args[1].parse().expect("Wanted a number");
    println!("fib({}) = {}", n, fib(n));
  }
}
