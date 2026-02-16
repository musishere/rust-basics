fn main() {
    let sum = my_name(21, 22);
    println!("This is sum: {}", sum)
}

fn my_name(x: i32, y: i32) -> i32 {
    println!("Another function with argumensts: {}", x);
    println!("Another function with argumensts: {}", y);

    let sum = x + y;
    return sum;
}
