//Remember : A vector allows you to store a variable number of values next to each other.
fn main() {
    //immutable vector
    println!("\nImmutable vector.");
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    println!("\nMutable vector, updating with push method.");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    println!("\nInitialization with vec! macro.");
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    println!("\nReading Element using index");
    let v = vec![1, 2, 3];
    let thrd: &i32 = &v[2];
    println!("3rd element of Vector ={:?} is {}.", v, thrd);

    println!("\nReading Element using get method");
    let v = vec![1, 2, 3];
    match v.get(2) {
        Some(third) => println!("3rd element of Vector ={:?} is {}.", v, third),
        None => println!("3rd element of Vector ={:?} is unreachable.", v),
    };
    match v.get(3) {
        Some(fourth) => println!("4th element of Vector ={:?} is {}.", v, fourth),
        None => println!("4th element of Vector ={:?} is unreachable.", v),
    };

    //Following code will not compile.
    //Recall the rule that states you can’t have mutable and immutable references
    //in the same scope.
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    println!("\nIterating through the veactor");
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    println!("\nIterating through the veactor (and mute elements)");
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    //Use enum to store multiple types
    #[derive(Debug)]
    enum CellType {
        Int(i32),
        Double(f64),
        Text(String),
    }

    let v = vec![
        CellType::Int(1),
        CellType::Double(42.1),
        CellType::Text(String::from("Bingo")),
    ];
    println!("{:?}", v);
}
