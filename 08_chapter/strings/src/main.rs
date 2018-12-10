fn main() {
    let slice = "This is string slice.";
    let mut s = slice.to_string();
    s.push_str(" Some string is pushed.");
    println!("{}", s);

    //Learning : pust_str does not take the ownership of its argument.
    //As a result, its argument (String Slice) is still accessible after push_str
    //s1 is a String
    let mut s1 = String::from("Hello, ");
    //s2 is a String Slice
    let s2 = "world!";
    //s1 is updated
    s1.push_str(s2);
    //new value of s1
    println!("S1 = {}", s1);
    //s2 is still accessible
    println!("S2 = {}", s2);
}
