//Notes on Message passing (channels) :
//One major tool Rust has for accomplishing message-sending concurrency is the channel,
//a programming concept that Rust’s standard library provides an implementation of.
//Analogy :
//You can imagine a channel in programming as being like a channel of water,
//such as a stream or a river. If you put something like a rubber duck or boat into a stream,
//it will travel downstream to the end of the waterway.

//A channel in programming has two halves: a transmitter and a receiver.
//The transmitter half is the upstream location where you put rubber ducks into the river,
//and the receiver half is where the rubber duck ends up downstream.
//One part of your code calls methods on the transmitter with the data you want to send,
//and another part checks the receiving end for arriving messages.
//A channel is said to be closed if either the transmitter or receiver half is dropped.

//The way Rust’s standard library implements channels means a channel can have multiple
//sending ends that produce values but only one receiving end that consumes those values.
//Imagine multiple streams flowing together into one big river:
//everything sent down any of the streams will end up in one river at the end.

use std::sync::mpsc; //mpsc stands for multiple producer, single consumer.
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    //ceate one more tx from cloning...
    let tx2 = mpsc::Sender::clone(&tx1);

    //we’re using thread::spawn to create a new thread and then using move to move tx
    //into the closure so the spawned thread owns tx. The spawned thread needs to own
    //the transmitting end of the channel to be able to send messages through the channel.
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi,"),
            String::from("1: how"),
            String::from("1: are"),
            String::from("1: you?"),
        ];
        for val in vals {
            //The send method returns a Result<T, E> type, so if the receiving end has already been
            //dropped and there’s nowhere to send a value, the send operation will return an error.
            //In this example, we’re calling unwrap to panic in case of an error.
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2: hi,"),
            String::from("2: how"),
            String::from("2: are"),
            String::from("2: you?"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
