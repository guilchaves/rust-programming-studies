fn main() {
    let my_vec = vec![1, 22, 54, 12, 88, 23, 5, 32];

    let num = highest_num(my_vec);

    println!("The highest number in vector is {}", num);
}

fn highest_num(vec: Vec<i32>) -> i32 {
    let mut highest_num = 0;

    for num in vec {
        if num > highest_num{
            highest_num = num;
        }
    }
    highest_num
}
