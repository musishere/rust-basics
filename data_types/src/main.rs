fn main() {
    println!("Hello, world!");

    // scaller types
    // integers, strings, boolean, floating, char

    // 1. integers(8bit,16,32) signed(i) or unsigned(u)
    let no = 2;
    println!("number is: {}", no);

    // 2. boolean
    let is_male = true;
    println!("{}", is_male);

    // 3. char

    let chart = "abc";
    println!("{}", chart);

    let dec = 78.99;
    println!("Floating number: {}", dec);

    // Compound types

    //arrays tuples dictionaries
    // 1.tuples
    let tup(i32, u8, f64) = (33, 56, 72.1);
    println!("{:?}", tup);
    println!("{}", tup.1);
}
