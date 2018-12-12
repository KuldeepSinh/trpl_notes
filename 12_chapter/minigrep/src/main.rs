use std::env;
//use std::error::Error;
//use std::fs;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    //Learning
    //iterators produce a series of values,
    //and we can call the collect method on an iterator to turn it into a collection,
    //such as a vector, containing all the elements the iterator produces.
    //let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //eprintln prints to stderr
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        //eprintln prints to stderr
        eprintln!("Application error {}.", e);
        process::exit(1);
    }
}
