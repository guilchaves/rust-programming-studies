use std::io;

fn main() {
    let mut factorial_input = String::new();

    io::stdin()
        .read_line(&mut factorial_input)
        .expect("Error reading input");

    let mut factorial = 1;
    let mut input_int = convert_to_int(&factorial_input);

    let input_string = format!("{}", input_int);

    while input_int > 1 {
        factorial = factorial * input_int;
        input_int = input_int - 1;
    }

    println!("{}! is {}", input_string, factorial);
}

fn convert_to_int(data_input: &String) -> i64 {
    let x = data_input.trim().parse::<i64>().unwrap();
    x
}
