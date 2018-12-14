//Notes on Threads :
//In most current operating systems, an executed program’s code is run in a process,
//and the operating system manages multiple processes at once. Within your program,
//you can also have independent parts that run simultaneously.
//The features that run these independent parts are called threads.

//Programming languages implement threads in a few different ways.
//Many operating systems provide an API for creating new threads.
//This model where a language calls the operating system APIs to create threads is sometimes
// called 1:1, meaning one operating system thread per one language thread.

//Many programming languages provide their own special implementation of threads.
//Programming language-provided threads are known as green threads,
//and languages that use these green threads will execute them in the context of a different
//number of operating system threads.
//For this reason, the green-threaded model is called the M:N model:
//there are M green threads per N operating system threads,
//where M and N are not necessarily the same number.

//The Rust standard library only provides an implementation of 1:1 threading.

use std::thread;
use std::time::Duration;

fn main() {
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("value of i from the spawned thread = {}.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //Example : Moving values from main thread to spawned thread.
    let v = vec![1, 2, 3];

    //note : use of move keyword
    let new_handle = thread::spawn(move || {
        println!("Value of vector v is moved to a thread is {:?}.", v);
    });

    for i in 1..5 {
        println!("value of i from the main thread = {}.", i);
        thread::sleep(Duration::from_millis(1));
    }
    join_handle.join().unwrap(); //join method will cause main thread wait for the spawned thread to finish.
    new_handle.join().unwrap();
}
