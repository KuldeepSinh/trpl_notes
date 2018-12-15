//strct is public, accessible from outside.
pub struct AveragedCollection {
    //fields are encapsulated keeping them private.
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    //following fn is private
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
}

fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(1);
    ac.add(2);
    ac.add(3);
    println!("Average = {:?}", ac.average());
    ac.remove();
    println!("Average = {:?}", ac.average());
}
