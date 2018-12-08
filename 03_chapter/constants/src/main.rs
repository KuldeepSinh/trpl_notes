//use of const:
//Constants are valid for the entire time a program runs,
//within the scope they were declared in.
//Following const is defined at the global scope.
const MAX_POINTS: u32 = 100_000;

fn main() {
    //print const
    println!("Value of the global const is {}.", MAX_POINTS);
    //print const from a function accessing const from the global scope.
    print_global_const();
    //print const from a function accessing const from the local scope.
    print_local_const();
}

//Following function accesses const defined at the global scope.
fn print_global_const() {
    println!("Value of the global const is {}.", MAX_POINTS);
}

//Following const is defined at the local scope.
fn print_local_const() {
    println!("Value of the local const is {}.", MAX_POINTS);
    const MAX_POINTS: u32 = 999_999;
}
