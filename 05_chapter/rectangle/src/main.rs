//Example Structures, methods and associated methods...
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

//Learning:
//1. Methods are the functions implemented on Structures, which take "self" as parameter.
//2. Associated functions implemented on Structures, which don't "self" as parameter.
impl Rectangle {
    //Following fns are methods of Rectangle
    //immutable borrow
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.length > other.length)
            || (self.width > other.length && self.length > other.width)
    }
}

impl Rectangle {
    //Following fn is an associated function of Rectangle
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    //Rectangle
    let rect1 = Rectangle {
        length: 10,
        width: 5,
    };

    let rect2 = Rectangle {
        length: 9,
        width: 4,
    };

    let rect3 = Rectangle {
        length: 4,
        width: 8,
    };
    println!("\nExamples of Structs and their methods.");
    println!("Area of {:?} is {} square units.", rect1, rect1.area());
    println!("Rect1 can hold Rect2 is {}.", rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect2 is {}.", rect1.can_hold(&rect3));
    println!("Rect2 can hold Rect3 is {}.", rect2.can_hold(&rect3));

    println!("\nExample of a Struct and its associated function.");
    println!("Square = {:?}", Rectangle::square(42));
    println!("Square now uses method to calculate its area.");
    println!("Area of our square is {}", Rectangle::square(42).area());
}
