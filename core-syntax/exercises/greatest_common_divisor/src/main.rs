fn main() {
    calculate_gct(15, 40);
}

fn calculate_gct (x: i32, y: i32) {

    let mut gct = x; 
    while  x % gct != 0 || y % gct != 0{
        gct = gct - 1;
    }
    
    println!("The greatest common divisor of {} and {} is {}", x, y, gct);
}
