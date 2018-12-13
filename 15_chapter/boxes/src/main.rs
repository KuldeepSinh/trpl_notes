//A pointer is a general concept for a variable that contains an address in memory.
// The most common kind of pointer in Rust is a reference.
//References are indicated by the & symbol and borrow the value they point to.

//Smart pointers, on the other hand, are data structures that not only act like
//a pointer but also have additional metadata and capabilities.

//In Rust, an additional difference between references and smart pointers is that
//references are pointers that only borrow data;
//in contrast, in many cases, smart pointers own the data they point to.

//Smart pointers are usually implemented using structs.
//The characteristic that distinguishes a smart pointer from an ordinary struct is that
//smart pointers implement the Deref and Drop traits.
//The "Deref" trait allows an instance of the smart pointer struct to behave like a
//reference so you can write code that works with either references or smart pointers.
//The "Drop" trait allows you to customize the code that is run when an instance of
//the smart pointer goes out of scope.

//The most straightforward smart pointer is a box, whose type is written Box<T>.
//Boxes allow you to store data on the heap rather than the stack.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
