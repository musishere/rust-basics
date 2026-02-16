fn main() {
    // 1.Each value in rust has a owner variable
    // 2.There can be only one owner at time
    // 3.When the owner goes out of the scope the value will be droped

    {
        // let s = "Learn Rust"; //Stored in stack
        let s = String::from("Learn Rust"); //Stored in Heap.
    }
}
