//Slices let you reference a contiguous sequence of elements in a collection
//rather than the whole collection.
fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; //include start - exclude end.
    let world = &s[7..12];
    println!("first slice = {}\nsecond slice = {}.", hello, world);

    let hello = &s[0..=4]; //include start & end both.
    let world = &s[7..=11];
    println!("\nfirst slice = {}\nsecond slice = {}.", hello, world);

    let hello = &s[..=4];
    println!("\nSlice to a point included = {}", hello);
    let hello = &s[..5];
    println!("Slice to a point excluded = {}", hello);

    let world = &s[7..];
    println!("\nSlice to the end = {}", world);
    let len = s.len();
    let world = &s[7..len];
    println!("Slice to the end = {}", world);

    let len = s.len();
    let whole_slice = &s[0..len];
    println!("\nWhole string as a slice = {}", whole_slice);
    let whole_slice = &s[..];
    println!("Whole string as a slice = {}", whole_slice);

    let s = String::from("Beautiful World");
    println!("\nFirst word = {}", first_word(&s));
}

//Following function uses slicing.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }
    &s[..]
}
