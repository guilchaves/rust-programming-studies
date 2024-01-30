pub enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    if let Ok(v) = a {
        println!("val = {}", v)
    }

    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Cannot divide by zero.".to_string());
    }
    Ok(x / y)
}
