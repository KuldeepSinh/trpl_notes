//Ownership Rules...
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

//The ownership of a variable follows the same pattern every time:
//  assigning a value to another variable moves it.
//When a variable that includes data on the heap goes out of scope,
//  the value will be cleaned up by "drop"
//  unless the data has been moved to be owned by another variable.
fn main() {
    println!("\nOwnership of variable with non-premitive types.");
    //s1 is instantiated and initialized.
    let s1 = String::from("Hello, world!");
    //value of s1 is assigned to s2.
    //s1 goets out of the scope.
    //s2 gets the ownership of the string.
    let s2 = s1;

    //Following line will not compile,
    //becuase previous line moved the value from s1 to s2,
    //as a result s1 went out of scope and cannot be used.
    //println!("{}", s1);
    println!("{}", s2);

    println!("\nCloning of values");
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone();
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    //ownership of variable with premitive types.
    println!("\nOwnership of variable with premitive types.");
    let x = 5;
    //here, x is of premitive type.
    //Rust copies value of x to y.
    //x never goes out of the scope.
    let y = x;
    println!("x = {}, y = {}.", x, y);

    println!("\nFor non-premitive types, ownership of variable is moved to the fn. parameters.");
    let s = String::from("Hello!");
    //s's value will be moved to the parameter of the takes_ownership() fn.
    takes_ownership(s);
    //as s's value has already been moved in previous fn. call,
    //following line will not compile.
    //println!("s = {}", s);

    let s = gives_ownership();
    println!("s = {}", s);

    let s = String::from("Hello!");
    let s = takes_and_gives_ownership(s);
    println!("s = {}", s);

    println!("\nFor premitive types, ownership of variable is NOT moved to the fn. parameters.");
    let i = 32;
    //i's value will be copied to the parameter of the makes_copy() fn.
    makes_copy(i);
    println!("i = {}", i);
}

fn takes_ownership(s: String) {
    println!("s = {}", s);
}

fn makes_copy(i: i32) {
    println!("i = {}.", i);
}

fn gives_ownership() -> String {
    let s = String::from("Hello!");
    s
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}
