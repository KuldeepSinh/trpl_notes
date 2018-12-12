//Rust’s closures are anonymous functions
//you can save in a variable or pass as arguments to other functions.
use std::thread;
use std::time::Duration;

//memoization:
//We can create a struct that will hold the closure and
//the resulting value of calling the closure.
//The struct will execute the closure only if we need the resulting value,
//and it will cache the resulting value so the rest of our code
//doesn’t have to be responsible for saving and reusing the result.
//You may know this pattern as memoization or lazy evaluation.

//To make a struct that holds a closure,
//we need to specify the type of the closure,
//because a struct definition needs to know the types of each of its fields.
//Each closure instance has its own unique anonymous type:
//that is, even if two closures have the same signature,
//their types are still considered different.
//To define structs, enums, or function parameters that use closures,
//we use generics and trait bounds

//The Fn traits are provided by the standard library.
//All closures implement at least one of the traits: Fn, FnMut, or FnOnce.

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    //The Cacher struct has a calculation field of the generic type T.
    //The trait bounds on T specify that it’s a closure by using the Fn trait.
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //Use of closure
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
