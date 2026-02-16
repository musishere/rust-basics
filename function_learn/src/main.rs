fn main() {
    println!("Hello, world!");
    first_fn();

    // passing single parameters
    second_fn(20);

    //passing multiple parameters
    third_fn(20, 'M');

    // function expression
    ex();

    // catch return value
    let x = return_value();
    println!("Returned value is: {}", x)
}

// simple function
fn first_fn() {
    println!("new function");
}

// pass simple parameter
fn second_fn(x: i32) {
    println!("The value of x is: {}", x);
}

// multiple parameter
fn third_fn(x: i32, y: char) {
    println!("The value of x is: {x} and the value of y is: {y}");
}

// expressions (example of statement)
fn ex() {
    let y = {
        let x = 9;
        x + 1
    };

    println!("Value of y is {}", y);
}

// return value from function

fn return_value() -> i32 {
    return 78 + 34;
}
