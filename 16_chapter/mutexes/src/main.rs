//Notes on Mutex
//Mutex is an abbreviation for mutual exclusion, as in,
//a mutex allows only one thread to access some data at any given time.
//To access the data in a mutex, a thread must first signal that it wants access
//by asking to acquire the mutex’s lock.
//The lock is a data structure that is part of the mutex that keeps track of
//who currently has exclusive access to the data.
//Therefore, the mutex is described as guarding the data it holds via
// the locking system.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //m is a mutex with some data (=5) inside.
    //Mutex<T> is a smart pointer.
    //More accurately, the call to lock returns a smart pointer called MutexGuard.
    //This smart pointer implements Deref to point at our inner data;
    //the smart pointer also has a Drop implementation that releases the lock
    //automatically when a MutexGuard goes out of scope,
    //which happens at the end of the following inner scope in Listing.
    let m = Mutex::new(5);
    {
        //To access the data inside the mutex, we use the lock method
        //to acquire the lock. This call will block the current thread
        //so it can’t do any work until it’s our turn to have the lock.
        //The call to lock would fail if another thread holding the lock panicked.
        let mut num = m.lock().unwrap();
        //After we’ve acquired the lock, we can treat the return value,
        //named num in this case, as a mutable reference to the data inside.
        *num = 6;
    }
    println!("m = {:?}", m);

    //Example : Multiple ownership with multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
