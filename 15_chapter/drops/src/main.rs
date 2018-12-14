//The Drop traig lets you customize what happens when a value is about to go out of
//scope. You can provide an implementation for the Drop trait on any type,
//and the code you specify can be used to release resources like files
//or network connections.

//The functionality of the Drop trait is almost always used when implementing
//a smart pointer.
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data = {}.", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };
    let d = CustomSmartPointer {
        data: String::from("world"),
    };

    let e = CustomSmartPointer {
        data: String::from("wonderful"),
    };
    //Rust doesn’t let you call the Drop trait’s drop method manually;
    //instead you have to call the std::mem::drop function provided by
    //the standard library if you want to force a value to be dropped
    //before the end of its scope.
    //Following line will not compile.
    //e.data();
    println!("===========");
    drop(e); //this will call drop method.
    println!("===========");

    println!("Custom smart pointers!");
} // c and d go out of scope, so drop will be called.
  //Variables are dropped in the reverse order of their creation,
  //so d was dropped before c.
