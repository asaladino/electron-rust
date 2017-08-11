mod fibonacci;
//extern crate tensorflow;

pub fn main() {
    fib();
//    tensor();
}

//fn tensor() {
//    let version = tensorflow::version();
//    match version {
//        Ok(v) => println!("TensorFlow version: {:?}", v),
//        Err(e) => println!("Error parsing header: {:?}", e),
//    }
//}

fn fib() {
    let result = fibonacci::calc(12);
    println!("Fibonacci: {}", result);
}