extern crate xml;

mod rssfeed;
mod parser;


// the #[tokio::main] is some async thing i don't know yet
#[tokio::main]
async fn main() {
    match rssfeed::RssFeed::new(String::from("test.xml")) {
        Ok(x) => {
            let mut rss_feed = x;
            rss_feed.add_source(String::from("http://feeds.washingtonpost.com/rss/rss_comic-riffs?itid=lk_inline_manual_53")).await;
            println!("{} {:?}", rss_feed.get_src_count(), rss_feed.get_src_name())
        }
        Err(x)=> {
            println!("Error! {}", x);
        }
    }
 
    parser::xml_parser();
}

