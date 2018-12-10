fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!(
        "Largest of the list {:?} is {}.",
        number_list,
        largest(&number_list)
    );

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!(
        "Largest of the list {:?} is {}.",
        char_list,
        largest(&char_list)
    );

    let int_point = Point { x: 5, y: 10 };
    println!("IntPoint = {:?}", int_point);

    let float_point = Point { x: 5.5, y: 10.5 };
    println!("FloatPoint = {:?}", float_point);
    println!("Value of x = {:?}", float_point.x());
    println!("Value of y = {:?}", float_point.y());
    println!(
        "Distance from origin = {:?}",
        float_point.distance_from_origin()
    );
}

use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
//generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
//type specific method
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.x.powi(2)).sqrt()
    }
}
