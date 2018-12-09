#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum MyOption<T> {
    MySome(T),
    MyNone,
}

fn plus_one(x: &MyOption<u32>) -> MyOption<u32> {
    match x {
        MyOption::MySome(x) => MyOption::MySome(x + 1),
        MyOption::MyNone => MyOption::MyNone,
    }
}

fn main() {
    //accessing enum value.
    let v4 = IpAddrKind::V4;
    println!("Ip Address v4 = {:?}", v4);
    let v6 = IpAddrKind::V6;
    println!("Ip Address v6 = {:?}", v6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("Home IP Address = {:?}", home);
    println!("Loop-back Address = {:?}", loopback);

    //Example of enum and pattern-matching.
    let coin = Coin::Quarter;
    println!("A Coin {:?} in cents = {}.", coin, coin.value_in_cents());
    println!(
        "A Coin {:?} in cents = {}.",
        Coin::Dime,
        Coin::Dime.value_in_cents()
    );

    println!(
        "A Coin {:?} in cents = {}.",
        Coin::Nickel,
        Coin::Nickel.value_in_cents()
    );

    println!(
        "A Coin {:?} in cents = {}.",
        Coin::Penny,
        Coin::Penny.value_in_cents()
    );

    println!(
        "A Coin {:?} in cents = {}.",
        Coin::Quarter,
        Coin::Quarter.value_in_cents()
    );

    println!("{:?}", MyOption::MySome("Truth."));
    let none: MyOption<String> = MyOption::MyNone;
    println!("{:?}", none);

    let my_num: MyOption<u32> = MyOption::MySome(41);
    println!("Value of {:?} + 1 = {:?}", my_num, plus_one(&my_num));
}
