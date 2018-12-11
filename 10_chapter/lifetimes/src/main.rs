//Lifetime is a scope for which a reference is valid.
//We must annotate types when multiple types are possible.
//In a similar way, we must annotate lifetimes
//when the lifetimes of references could be related in a few different ways.

//Main aim of lifetimes is to prevent dangling referenes,
//which cause a program to reference data other than the data it's indended to reference.

//Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
//Once they’re connected, Rust has enough information to allow memory-safe operations and
//disallow operations that would create dangling pointers or otherwise violate memory safety.

fn main() {
    //Example#1
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    println!("The longest of {} and {} is {}.", str1, str2, result);

    //Example#2
    let str1 = String::from("A very long string.");
    {
        let str2 = String::from("Short String.");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest of - {} and - {} is - {}", str1, str2, result);
    }

    //Example#3 Following example will not compile.
    //The example shows that ...
    //for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope.
    // let str1 = String::from("Very long String.");
    // let result;
    // {
    //     let str2 = String::from("Short String."); //scope of str2 starts here.
    //     result = longest(str1.as_str(), str2.as_str());
    // } //scope str2 ends here.
    // println!("The longest string is {}.", result); //result may refer to str2 : but its already out of scope here.

    //example#4 lifetime in struct and its mathod.
    let str1 = String::from("hello");
    let struct_val = ImportantExcerpt {
        part: str1.as_str(), //part is a refereces to str1.
    };
    println!("Value gathered from struct = {:?}.", struct_val.get());

    //Example#5 : Static Lifetime
    //One special lifetime we need to discuss is 'static,
    //which denotes the entire duration of the program.
    //All string literals have the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    println!("Static Lifetime - {:?}", s);
}

//-------lifetime in functions
//The function signature now tells Rust that ...
//1. For some lifetime 'a, the function takes two parameters,
//   both of which are string slices that live at least as long as lifetime 'a.
//2. The string slice returned from the function will live at least as long as lifetime 'a.
//When annotating lifetimes in functions, the annotations go in the function signature, not in the function body

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    match str1.len() > str2.len() {
        true => str1,
        false => str2,
    }
}

// Example of a dangling reference. Following code will not compile.
// "result" is created within the function. And it will not exist after the function.
// fn longest_wrong<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string"); //scope of "result" starts here.
//     result.as_str()
// } //scope of "result" end here and gets cleaned up.

//lifetime in structs
// It’s possible for structs to hold references,
//but in that case we would need to add a lifetime annotation
//on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
//lifetime in method definition
impl<'a> ImportantExcerpt<'a> {
    pub fn get(&self) -> &str {
        self.part
    }
}
