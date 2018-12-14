//A pointer is a general concept for a variable that contains an address in memory.
// The most common kind of pointer in Rust is a reference.

//"References" are indicated by the & symbol and borrow the value they point to.
//"Smart pointers", on the other hand, are data structures that not only act like
//a pointer but also have additional metadata and capabilities.

//In Rust, an additional difference between references and smart pointers is that
//references are pointers that only borrow data;
//in contrast, in many cases, smart pointers own the data they point to.

//Smart pointers are usually implemented using structs.
//The characteristic that distinguishes a smart pointer from an ordinary struct is that
//smart pointers implement the "Deref" and "Drop" traits.
//The "Deref" trait allows an instance of the smart pointer struct to behave like a
//reference so you can write code that works with either references or smart pointers.
//The "Drop" trait allows you to customize the code that is run when an instance of
//the smart pointer goes out of scope.

//The most straightforward smart pointer is a Box, whose type is written Box<T>.
//Boxes allow you to store data on the heap rather than the stack.

use crate::List::{Cons, Nil};
use crate::NumList::{ICons, INil};

fn main() {
    //Prints 5.
    let b = Box::new(5);
    println!("b = {}", b);

    //Prings the num-list
    let numlist: NumList = ICons(1, Box::new(ICons(2, Box::new(ICons(3, Box::new(INil))))));
    println!("{:?}", numlist);

    let list: List<String> = Cons(
        String::from("Hello"),
        Box::new(Cons(
            String::from("Wonderful"),
            Box::new(Cons(String::from("World"), Box::new(Nil))),
        )),
    );
    println!("Generic List {:?}", list);
}

//Creating a recursive type :
//At compile time, Rust needs to know how much space a type takes up.
//One type whose size can’t be known at compile time is a recursive type,
//where a value can have as part of itself another value of the same type.
//Because this nesting of values could theoretically continue infinitely,
//Rust doesn’t know how much space a value of a recursive type needs.
//However, boxes have a known size,
//so by inserting a box in a recursive type definition, you can have recursive types.
#[derive(Debug)]
enum NumList {
    ICons(i32, Box<NumList>),
    INil,
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
