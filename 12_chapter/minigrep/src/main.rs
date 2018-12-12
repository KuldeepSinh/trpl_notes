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
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    //println!("Searching for {}.", &config.query);
    //println!("In file {}.", &config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}.", e);
        process::exit(1);
    }
}
