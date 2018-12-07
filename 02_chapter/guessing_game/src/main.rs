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

    //User of rand
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        //refer : https://doc.rust-lang.org/std/macro.print.html
        print!("Please input your guess: ");
        io::stdout().flush().expect("Flushed.");

        //Read user input into a mutable variable of type String
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //parse to u32.
        //Learn : We are using shadowing of guess with let.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Learn : use of match
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
