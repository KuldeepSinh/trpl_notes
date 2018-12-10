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

    //Creating HasMaps using Collect method from Zipping 2 Vectors
    let teams = vec![
        String::from("Blue"),
        String::from("Geen"),
        String::from("Red"),
    ];
    let scores = vec![10, 20, 15];
    let team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("Team Scores = {:?}", team_scores);

    //creating from a Vector of Tuples (Option#1)
    let team_scores: HashMap<String, i32> = vec![
        (String::from("Blue"), 10),
        (String::from("Gren"), 20),
        (String::from("Red"), 15),
    ]
    .iter()
    .cloned() //Read : Reference => https://stackoverflow.com/questions/32354947/type-issue-with-iterator-collect
    .collect();
    println!("Team Scores = {:?}", team_scores);

    //creating from a Vector of Tuples (Option#2)
    let team_scores: HashMap<String, i32> = vec![
        (String::from("Blue"), 10),
        (String::from("Gren"), 20),
        (String::from("Red"), 15),
    ]
    .into_iter() //Read : Reference => https://stackoverflow.com/questions/32354947/type-issue-with-iterator-collect
    .collect();
    println!("Team Scores = {:?}", team_scores);

    //Accessing HashMap Element with get method
    let my_flag = flags.get(&key);
    println!("{:?}", my_flag);

    //Accessing with iter
    for pair in &team_scores {
        println!("{:?}", pair);
    }
    for (key, value) in &team_scores {
        println!("{}: {}", key, value);
    }

    //In a HashMap each key can be associated with only one value.
    //Updating Option # 1: Overwrite
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue"), 15);
    println!("Scores = {:?}", scores);

    //Updating Option # 2: Only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("Scores = {:?}", scores);

    //Updating Option # 3: Updating based on old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        //The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
        //Here we store that mutable reference in the count variable,
        //so in order to assign to that value, we must first dereference count using the asterisk (*).
        *count += 1;
        //The mutable reference goes out of scope at the end of the for loop,
        //so all of these changes are safe and allowed by the borrowing rules.
    }
    println!("{:?}", map);
}
