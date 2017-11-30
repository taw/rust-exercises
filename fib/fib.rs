fn fib(n : u32) -> u32 {
  if n <= 1 {
    return n;
  } else {
    return fib(n-1) + fib(n-2);
  }
}

// Expect fib(10) = 55
fn main() {
  let n = 10;
  println!("fib({}) = {}", n, fib(n));
}
