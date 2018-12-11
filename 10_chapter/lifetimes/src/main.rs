//lifetime is a scope for which a reference is valid.
//We must annotate types when multiple types are possible.
//In a similar way, we must annotate lifetimes
//when the lifetimes of references could be related in a few different ways.

//Main aim of lifetimes is to prevent dangling referenes,
//which cause a program to reference data other than the data it's indended to reference.
fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    println!("The longest of {} and {} is {}.", str1, str2, result);
}

//lifetime in functions
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    match str1.len() > str2.len() {
        true => str1,
        false => str2,
    }
}

//lifetime in structs
// It’s possible for structs to hold references,
//but in that case we would need to add a lifetime annotation
//on every reference in the struct’s definition.
