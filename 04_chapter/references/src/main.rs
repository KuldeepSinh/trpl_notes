//At any given time,
//  you can have either (but not both of) one mutable reference or
//  any number of immutable references.
//References must always be valid.
fn main() {
    println!("\nImmutable reference example.");
    let mut s1 = String::from("Hello!");
    //We are passing an immutable reference to the function called below.
    let len = calculate_length(&s1);
    println!("Length of {} is {}.", s1, len);

    println!("\nMutable reference example.");
    change_string(&mut s1);
    println!("s1 after mutation : {}", s1);
}
//Immutable reference example.
//Following fn borrows value from its caller.
//Having references as function parameters is called borrowing.
fn calculate_length(s: &String) -> usize {
    s.len()
}
//Mutable reference example.
//Having references as function parameters is called borrowing.
fn change_string(s: &mut String) {
    s.push_str(" world!")
}

//Compiler gives errors when Dangling references are made.
//Following code will not compile.
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// } //s goes out of scope here. so its reference will not be valid after this point.
