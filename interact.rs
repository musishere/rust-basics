fn main() {
    let x = 5;
    let y = x; //Copy

    let name = String::from("Mustafa");
    let s2 = name.clone(); // Move not shallow copy

    println!("{}", name);
}
