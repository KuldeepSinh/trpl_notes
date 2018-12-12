#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //note :
    //1. use of into_iter fn
    //2. The closure captures value of shoe size from its environment,
    //exteranl to the paramter passsed to it.
    //3. collect() gathers the values returned by the adapted iterator into a vector.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}
//Implementation of Iterator.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    #[test]
    fn iterator_demostration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    //The Iterator trait has a number of different methods with default implementations
    //provided by the standard library. Some of these methods call the next method in their
    //definition, which is why you’re required to implement the next method
    //when implementing the Iterator trait. Methods that call next are called
    //"consuming adaptors", because calling them uses up the iterator.

    //One example is the sum method,
    //which takes ownership of the iterator and iterates through the items by
    //repeatedly calling next, thus consuming the iterator.
    //We aren’t allowed to use v1_iter after the call to sum
    //because sum takes ownership of the iterator we call it on.

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    //Other methods defined on the Iterator trait, known as "iterator adaptors",
    //allow you to change iterators into different kinds of iterators.
    //You can chain multiple calls to iterator adaptors to perform complex actions
    //in a readable way.
    //But because all iterators are lazy,
    //you have to call one of the consuming adaptor methods to get results
    //from calls to iterator adaptors.
    #[test]
    fn iterator_map_collect() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    //The filter method on an iterator takes a closure
    //that takes each item from the iterator and returns a Boolean.
    //If the closure returns true,
    //the value will be included in the iterator produced by filter.
    //If the closure returns false,
    //the value won’t be included in the resulting iterator
    #[test]
    fn iterator_filter() {
        use super::*;
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 9,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }

    #[test]
    fn iterator_implementation() {
        use super::*;

        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
