use std::io;

fn main() {
    let mut input = String::new();

println!("Enter number to find fibonacci");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    let position = input.trim().parse::<u32>().unwrap();
    
    let result = fibonacci(position);

    println!("The Fibonacci number in position {} is {}", position, result);
}

fn fibonacci(n: u32) -> f64 {
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    (phi.powi(n as i32) / 5.0_f64.sqrt()).round()
}
