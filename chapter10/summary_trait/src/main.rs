use summary_trait::Summary;
use summary_trait::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("Peter J"),
        content: String::from("여러분 Rust하자 Rust!!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}