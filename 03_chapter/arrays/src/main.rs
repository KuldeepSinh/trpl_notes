fn main() {
    print_arrays();
}

fn print_arrays() {
    let bs: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Arraya bs = {:?}.", bs);

    let fst_quarter = ["Jan", "Feb", "Mar"];
    println!("First quarter = {:?}.", fst_quarter);
}
