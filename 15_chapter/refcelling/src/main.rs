//RecCell Notes:
//Interior mutability (a mutable borrow to an immutable value) is a design pattern in Rust that allows you to mutate data
//even when there are immutable references to that data; normally, this action is disallowed
//by the borrowing rules.
//To mutate data, the pattern uses "unsafe" code inside a data structure to bend Rust’s usual rules
//that govern mutation and borrowing. We can use types that use the interior mutability pattern
//when we can ensure that the borrowing rules will be followed at runtime, even though
//the compiler can’t guarantee that. The unsafe code involved is then wrapped in a safe API,
//and the outer type is still immutable.

//Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
//With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
//With RefCell<T>, these invariants are enforced at runtime.
//With references, if you break these rules, you’ll get a compiler error.
//With RefCell<T>, if you break these rules, your program will panic and exit.

//Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and
//will give you a compile-time error if you try using it in a multithreaded context.

//1. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
//2. Box<T> allows immutable or mutable borrows checked at compile time;
//   Rc<T> allows only immutable borrows checked at compile time;
//   RefCell<T> allows immutable or mutable borrows checked at runtime.
//3. Because RefCell<T> allows mutable borrows checked at runtime,
//   you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

fn main() {
    //A consequence of the borrowing rules is that when you have an immutable value,
    //you can’t borrow it mutably.
    //For that reason, following code will not compile.
    // let x = 5; //here, x is not defined as mutable
    // let y = &mut x;
}

//Use Case : Mock objects
//A test double is the general programming concept for a type used in place of another type during testing.
//Mock objects are specific types of test doubles that record what happens during a test
//so you can assert that the correct actions took place.
//Rust doesn’t have objects in the same sense as other languages have objects,
//and Rust doesn’t have mock object functionality built into the standard library
//as some other languages do. However, you can definitely create a struct that will serve
//the same purposes as a mock object.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning : You have used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning : You have  used over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("You are over your limit")
        }
    }
}
