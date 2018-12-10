//Rust doesn't have exceptions.
//It provides Result<T, E> to handle recoverable errors.
//And provides a panic! macro, to stop execution when the program encounters an unrecoverable error.
//Wthn the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
fn main() {
    //Explicitly call panic!
    //panic!("Crash and Burn!");

    //Index out of bound will cause panic
    let v = vec![1, 2, 3];
    v[99];
}
