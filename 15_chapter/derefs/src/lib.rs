//Implementing the "Deref" trait allows you to customize the behavior of
//the dereference operator, * (as opposed to the multiplication or glob operator).
//By implementing "Deref" in such a way that a smart pointer can be treated
//like a regular reference, you can write code that operates on references and
//use that code with smart pointers too.
#[cfg(test)]
#[test]
fn deref_test() {
    let x = 42;
    let y = &x; //y is a reference
    let z = Box::new(x); //smart pointer

    assert_eq!(42, x);
    assert_eq!(42, *y); //dereference
    assert_eq!(42, *z);

    let y = MyBox::new(x);
    //*y has been made possible by implementiong "Deref"
    //When we entered *y in following Listing,
    assert_eq!(42, *y); //behind the scenes Rust actually ran this code: *(y.deref())
    assert_eq!(42, *(y.deref()));

    //Example deref coercion (see fn hello)
    let s = MyBox::new(String::from("hello"));
    hello(&s);
    //without deref coercion we would have to write as follows.
    hello(&(*s)[..]);
    //Anatomy of above line...
    //The (*m) dereferences the MyBox<String> into a String.
    //Then the & and [..] take a string slice of the String (.. means whole string)
    //that is equal to the whole string to match the signature of hello
}

//Reminder: Following is a Tuple-Struct
//and fields can be accessed using its ordinal position
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    //The "type Target = T;"
    //syntax defines an "associated type" for the Deref trait to use.
    type Target = T;

    //We fill in the body of the deref method with &self.0
    //so deref returns a reference to the value we want to access with the * operator.
    //The deref method gives the compiler the ability to take a value of any type
    //that implements Deref and call the deref method to get a & reference
    //that it knows how to dereference.

    //The reason the deref method returns a reference to a value,
    //and that the plain dereference outside the parentheses in *(y.deref()) is still
    //necessary, is the ownership system.
    //If the deref method returned the value directly instead of a reference
    //to the value, the value would be moved out of self.
    //We don’t want to take ownership of the inner value inside MyBox<T> in this case
    //or in most cases where we use the dereference operator.
    fn deref(&self) -> &T {
        &self.0
    }
}

//Deref Coercion:
//Deref coercion is a convenience that Rust performs on arguments to functions and
//methods.
//Deref coercion converts a reference to a type that
//implements Deref into a reference to a type
//that Deref can convert the original type into.

//Deref coercion happens automatically when we pass a reference to a particular type’s
//value as an argument to a function or method that doesn’t match
//the parameter type in the function or method definition.

//A sequence of calls to the deref method converts the type we provided into
//the type the parameter needs.

//Example deref coercion
pub fn hello(name: &str) {
    assert_eq!("hello", name);
}

//When the Deref trait is defined for the types involved,
//Rust will analyze the types and use Deref::deref as many times as necessary
//to get a reference to match the parameter’s type.
//The number of times that Deref::deref needs to be inserted
//is resolved at compile time,
//so there is no runtime penalty for taking advantage of deref coercion!
