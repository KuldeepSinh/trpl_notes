fn main() {
    //In Rust, varaibles are immutable by default.
    //When a variable is immutable,
    //once a value is bound to a name, you can’t change that value.
    let x = 7;
    println!("Value of x is {}.", x);

    //Following line if uncommented will not compile.
    // x = 6;

    //In Rust, variables are declared as mutable using 'let mut'
    let mut y = 5;
    println!("Value of y before muation is {}.", y);
    y = y + 37;
    println!("Value of y after mutation is {}.", y);

    //shadowing
    //shadowing allows both value and type of the variable be changed.
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Nnumber of spaces = {}.", spaces);

    //print array
    print_array();

    //function with parameters.
    let a = 10;
    let b = 20;
    print_sum(a, b);
}

fn print_array() {
    let bs: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Arraya as = {:?}.", bs);
}

fn print_sum(a: u32, b: u32) {
    println!("Sum of a={} and b={} is {}.", a, b, a + b);
}
