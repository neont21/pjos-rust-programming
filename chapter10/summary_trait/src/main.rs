use summary_trait::{Tweet, NewsArticle};

fn main() {
    let _tweet = Tweet {
        username: String::from("Peter J"),
        content: String::from("여러분 Rust하자 Rust!!"),
        reply: false,
        retweet: false,
    };

    //println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Peter broken Hazzi's Windows"),
        location: String::from("Gwangjin-goo, Seoul, Korea"),
        author: String::from("Ferris"),
        content: String::from("Finally, Peter broken Hazzi's Windows, \
            and install Linux on Hazzi's laptop."),
    };

    //println!("New article available! {}", article.summarize());

    summary_trait::notify(&article);
}