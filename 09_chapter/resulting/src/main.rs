// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    //File open returns Result.
    let f = File::open("hello.txt");
    //handling the Result with pattern matching
    match f {
        Ok(file) => file,
        Err(error) => panic!("There was an error opening the file: {:?}", error),
    };

    //An alternative of match pattern above : unwrap method
    //unwrap, is a shortcut method that is implemented just like the match statement above.
    //If the Result value is the Ok variant,
    //unwrap will return the value inside the Ok.
    //If the Result is the Err variant, unwrap will call the panic! macro for us
    let f = File::open("hello.txt").unwrap();
    println!("Result = {:?}", f);

    //Another method, expect, which is similar to unwrap, lets us also choose the panic! error message.
    //Using expect instead of unwrap and providing good error messages can convey your intent
    //and make tracking down the source of a panic easier.
    let f = File::open("hello.txt").expect("Failed to open hello.txt file.");
    println!("Result = {:?}", f);

    //Using ? to propagate errors
    //The ? operator can only be used in functions that return Result
    let f = read_username_from_file();
    println!("Result = {:?}", f);

    //Using ? to propagate errors
    let f = read_username_from_file_shorter();
    println!("Result = {:?}", f);
}

//Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    //When you put ? after a Result value ...
    //If the value of Result is OK, then the vlaue inside teh Ok will be returned from the expression
    //If the value of Result is Er, then Err will be returned from the whole funcion
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    //above 3 lines of code can be shartened as follows....
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

//Rust provides a convenience function called fs::read_to_string that will
//open the file, create a new String, read the contents of the file, and put the contents into that String, and then return it.
use std::fs;
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
