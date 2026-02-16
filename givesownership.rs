fn main() {
    let takes_ownership = gives_ownership();

    println!("This is what ownership we got: {}", takes_ownership);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello world!");
    return some_string;
}
