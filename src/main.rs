mod fibonacci;

pub use fibonacci::calc;

pub fn main() {
    fib();
}

fn fib() {
    let result = fibonacci::calc(12);
    println!("Fibonacci: {}", result);
}