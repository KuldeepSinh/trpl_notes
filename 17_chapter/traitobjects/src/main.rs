pub trait Draw {
    fn draw(&self);
}

//The "Screen" struct holds a vector named "components".
//This vector is of type "Box<dyn Draw>"", which is a trait object;
//it’s a stand-in for any type inside a Box that implements the Draw trait.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//On the Screen struct, we’ve defined a method named run that will
//call the draw method on each of its components, as shown in following Listing.
//This works differently than defining a struct that uses a generic type parameter
//with trait bounds. A generic type parameter can only be substituted
//with one concrete type at a time, whereas trait objects allow for multiple concrete
//types to fill in for the trait object at runtime.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//implementing the trait
pub struct Button {}
impl Draw for Button {
    fn draw(&self) {
        println!("This is a button.");
    }
}

pub struct DropBox {}
impl Draw for DropBox {
    fn draw(&self) {
        println!("This is a drop-box.");
    }
}

pub fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {}),
            Box::new(DropBox {}),
            Box::new(Button {}),
        ],
    };
    screen.run()
}
