use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");

    let mut s = String::new();
    let banner = "Digite uma sequencia de números \
        separados por vírgula \
        exemplo: 1, 2, 3, 4, 5";

    println!("{banner}");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    let nums: Vec<i32> = s.split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();

    println!("O total é {}", result);

    println!("{}", "-".repeat(40));
}
