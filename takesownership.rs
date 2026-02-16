fn main() {
    let s = String::from("Random string is coming yay!");
    takes_ownership(s.clone());
    println!("{}", s);
}

fn takes_ownership(some_string: String) {
    println!("This is some random string: {}", some_string);
}
