#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Examples of tuple-structs
//Note : Color and Point are both different types.

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

//Example Rectangle
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

fn main() {
    let bob = User {
        username: String::from("Bob"),
        email: String::from("bob@email.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("Bob = {:?}", bob);
    //accessing individual elements
    println!("Bob's Email = {}", bob.email);

    //Make entire struct mutable,
    //Rust does not allow individual field of the struct to be mutable.
    let mut mutable_bob = User {
        username: String::from("Bob"),
        email: String::from("bob@email.com"),
        sign_in_count: 1,
        active: true,
    };
    mutable_bob.email = String::from("mutablebob@email.com");
    mutable_bob.active = false;
    mutable_bob.sign_in_count = 0;
    println!("Mutable Bob = {:?}", mutable_bob);

    let alice = build_user(String::from("Alice"), String::from("alice@email.com"));
    println!("Alice = {:?}", alice);

    //tuple-structs
    let black = Color(0, 0, 0);
    println!("Black = {:?}", black);
    let origin = Point(0, 0, 0);
    println!("Origin = {:?}", origin);

    //Rectangle
    let rect = Rectangle {
        length: 7,
        width: 6,
    };
    println!("Area of {:?} is {} square units.", rect, area(&rect));
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
