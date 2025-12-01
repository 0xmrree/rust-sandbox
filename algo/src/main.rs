mod fib;
use fib::fib;

fn main() {
    let result = fib(10);
    println!("fib(10) = {}", result);
}
