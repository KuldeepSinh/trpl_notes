//Analogy:
//Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on.
//Others can come into the room and watch the TV. When the last person leaves the room,
//they turn off the TV because it’s no longer being used.
//If someone turns off the TV while others are still watching it,
//there would be uproar from the remaining TV watchers!

//We use the Rc<T> type when we want to allocate some data on the heap for multiple parts
//of our program to read and we can’t determine at compile time which part
//will finish using the data last.
//In other words, Using Rc<T> allows a single value to have multiple owners,
//and the count ensures that the value remains valid as long as any of the owners still exist.
//Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.

//Note that Rc<T> is only for use in single-threaded scenarios.

//Example
#[allow(dead_code)]
enum BoxedList {
    Cons(i32, Box<BoxedList>),
    Nil,
}
#[allow(unused_imports)]
use crate::BoxedList::{Cons, Nil};
use crate::RcedList::{RcedCons, RcedNil};

#[allow(unused)]
fn main() {
    //We cannot use a value after its moved, following code will not compile.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); //a's value is moved here (now b owns the data)
    // let c = Cons(4, Box::new(a)); //we can not use a again as its already moved.

    let a = Rc::new(RcedCons(5, Rc::new(RcedCons(10, Rc::new(RcedNil)))));
    println!("Count after creating a = {}.", Rc::strong_count(&a));
    //Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase,
    //and the data won’t be cleaned up unless there are zero references to it.
    //The call to Rc::clone only increments the reference count.
    let b = RcedCons(3, Rc::clone(&a)); //Notice &a here: We are cloning the reference, Not the entire list.
    println!("Count after creating b = {}.", Rc::strong_count(&a));
    {
        let c = RcedCons(4, Rc::clone(&a));
        println!("count after creating c = {}.", Rc::strong_count(&a));
    }
    println!(
        "count after c goes out of scope = {}.",
        Rc::strong_count(&a)
    );
}

use std::rc::Rc;
#[derive(Debug)]
enum RcedList {
    RcedCons(i32, Rc<RcedList>),
    RcedNil,
}
