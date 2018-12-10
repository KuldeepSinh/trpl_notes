//The type HashMap<K, V> stores a mapping of keys of type K to values of type V.
//It does this via a hashing function,
//which determines how it places these keys and values into memory.
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 20);
    scores.insert(String::from("Red"), 15);
    println!("{:?}", scores);

    //Learning : HashMap and Ownership
    //For types that implement the Copy trait, like i32,
    //the values are copied into the hash map.
    //For owned values like String,
    //the values will be moved and the hash map will be the owner of those values.
    let key = String::from("India");
    let value = String::from("Tri Color");
    let mut flags = HashMap::new();
    flags.insert(key, value);
    println!("{:?}", flags);
    //values of key and value are moved to the hashmap.
    //Following lines will not compile.
    // println!("Key = {}", key);
    // println!("Value = {}", value);

    //Learning : HashMap and Ownership
    //If we insert references to values into the hash map,
    //the values won’t be moved into the hash map.
    //The values that the references point to must be valid for at least
    //as long as the hash map is valid.
    let key = String::from("India");
    let value = String::from("Tri Color");
    let mut flags = HashMap::new();
    flags.insert(&key, &value);
    println!("{:?}", flags);
    println!("Key = {}", key);
    println!("Value = {}", value);

    //Accessing HashMap Element
    let my_flag = flags.get(&key);
    println!("{:?}", my_flag);
}
