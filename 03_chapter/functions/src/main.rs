fn main() {
    //function with no parameters.
    print_array();

    //function multiple parameters.
    let a = 10;
    let b = 20;
    print_sum(a, b);

    //function retruning value
    println!("9 plus 1 is {}.", plus_one(9));
}

fn print_array() {
    let bs: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Arraya as = {:?}.", bs);
}

fn print_sum(a: u32, b: u32) {
    println!("Sum of a={} and b={} is {}.", a, b, a + b);
}

//if function returns, its return type is mandatory to declare.
fn plus_one(x: i32) -> i32 {
    x + 1
    //dont put semi-colon when function returns value as follows.
    // x + 1;
    // above code will not compile,
    // when you put ; at the end, the expression turns into a statement
}
