use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {

    let mut sum = 0;
    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Error reading input value");

    let mut value_i32 = convert_to_int(&input_value);

    while value_i32 != 0 {
        let r = value_i32 % 10;
        sum += r;
        value_i32 = value_i32 / 10;
    }

    println!("The sum of all digits are {}", sum);
}
