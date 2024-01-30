fn main() {
    let n = 10;
    let mut i = 1;

    while i <= n {
        println!("{}", i);
        i += 1;
    }
    println!("\n{}\n", "=".repeat(40));

    for j in 1..=10 {
        println!("{}", j);
    }
}
