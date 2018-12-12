//The iterator pattern allows you to perform some task on a sequence of items in turn.
//An iterator is responsible for the logic of iterating over each item
//and determining when the sequence has finished.

//In Rust, iterators are lazy, meaning they have no effect
//until you call methods that consume the iterator to use it up.
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); //lazy : nothing happens until this point.

    //values are accessed here.
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
