//Remember :
//A trait tells the Rust compiler about functionality a particular type has and can share with other types.
//We can use traits to define shared behavior in an abstract way.
//We can use trait bounds to specify that a generic can be any type that has certain behavior.

//Remember :
//One restriction to note with trait implementations is that
//we can implement a trait on a type only if
//either the trait or the type is local to our crate.

//Example
pub trait Summary {
    //Fn with default implementation.
    fn summarize(&self) -> String {
        format!("Read more from ... {}", self.author())
    }
    fn author(&self) -> String;
}

//Side Note : Struct is public and all its fields are public too.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}).", self.headline, self.author, self.location)
    }
    fn author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn author(&self) -> String {
        format!("{}", self.username)
    }
}
pub struct Note {
    pub writer: String,
    pub note: String,
}

impl Summary for Note {
    fn author(&self) -> String {
        format!("{}", self.writer)
    }
}
