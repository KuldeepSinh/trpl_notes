use traits::NewsArticle;
use traits::Note;
use traits::Summary;
use traits::Tweet;

fn main() {
    let article = NewsArticle {
        headline: String::from("India won!"),
        location: String::from("Aidelade"),
        author: String::from("CricInfo"),
        content: String::from("India won 1st test vs Aus by 32 runs."),
    };
    println!("News Summary : {:?}", article.summarize());

    let tweet = Tweet {
        username: String::from("CricInfo"),
        content: String::from("India won!"),
        reply: false,
        retweet: false,
    };
    println!("Tweet Summary : {:?}", tweet.summarize());

    let note = Note {
        writer: String::from("CricInfo"),
        note: String::from("Inida Wins"),
    };
    println!("Note summary : {:?}", note.summarize());

    println!("\nExample of traits as an argument.");
    notify(&article);
    notify(&tweet);
    notify(&note);

    println!("\nExample of traits as an argument.");
    notify_clone(&article);
    notify_clone(&tweet);
    notify_clone(&note);
}

//trait as argument : with syntatic sugar
fn notify(item: &impl Summary) {
    println!("Breaking : {:?}", item.summarize());
}
//trait as argument : without syntatic sugar
fn notify_clone<T: Summary>(item: &T) {
    println!("Breaking : {:?}", item.summarize());
}
