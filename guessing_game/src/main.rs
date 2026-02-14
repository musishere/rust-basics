use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Your secret number: {}",secret_number);

    loop{
        println!("Input your number");

        // mut tells rust that it is mutable Note: by default it is not mutable
        let mut guess = String::new();

        // input
        io::stdin().read_line(&mut guess).expect("Failed to readline");


        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}",guess);


        match guess.cmp(&secret_number){
            Ordering::Less=> println!("{}","Too small".red()),
            Ordering::Greater=> println!("Too Big"),
            Ordering::Equal=> {
                println!("You win");
                break;
            }
        }
    }
   
}
