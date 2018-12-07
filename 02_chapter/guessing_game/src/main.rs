//rand is an extranal dependency.
//Check Cargo.toml to see how cargo manages dependencies.
use rand::Rng;

//Use of std, comes along with Rust.
//Not required to be managed from Cargo.toml, explicitly.
use std::cmp::Ordering;
use std::io::{self, Write};

//main is the entry point.
fn main() {
    println!("Guess a number.");

    //Learn :
    //1. how variable are defined using let
    //2. In Rust, variables are immutable by default.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        //refer : https://doc.rust-lang.org/std/macro.print.html
        print!("Please input your guess: ");
        io::stdout().flush().expect("Flushed.");

        //Read user input into a mutable variable of type String
        //Learn: Use of "let mut" to define mutable variable.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //Learn : Use of shadowing to hide previous definition of variable used with same name.
        let guess: u32 = match guess.trim().parse() {
            //Learn: User of Result type to handle error conditions.
            Ok(num) => num,
            Err(_) => continue,
        };

        //Learn : Use of match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
