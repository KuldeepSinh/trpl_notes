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
