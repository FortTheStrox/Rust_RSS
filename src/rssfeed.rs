use std::fs::{File, write};

pub struct RssFeed {
    file: File,
    src_count: u32,
    src_names: Vec<String>,
}

impl RssFeed {
    pub fn new(file_name: String) -> Result<RssFeed, &'static str> {
        println!("Checking top directory of rust/rss_feed...");
        match File::open(file_name) {
            Ok(x) => {
                let file = x;
                println!("Good");
                let src_count = 0;
                let src_names = vec![];
                Ok(RssFeed{ file,src_count,src_names })
            }
            Err(x) => {
                println!("{}", x);
                return Err("Error opening");
            }
        }
    }
    // Getters
    pub fn get_src_count(&self) -> &u32 {
        &self.src_count
    }

    pub fn get_src_name(&self) -> &Vec<String> {
        &self.src_names
    }

    // RssFeed functions
    pub async fn add_source(&mut self, source: String) {
        self.set_src_count(1);
        self.set_src_names(&source);
        let body = test(source).await.unwrap();
        write("test.xml", body).unwrap();
    }

    // Setters
    fn set_src_count(&mut self, value: u32) {
        self.src_count += value;
    }

    // be careful i had to set the source to a borrow string, but i needed to clone source when pushed into the src_name vector because it would try to own the string
    fn set_src_names(&mut self, source: &String) {
        self.src_names.push(source.clone());
    }


}

// this function gets the response body of the xml, from the rss url
async fn test(source: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(source.as_str()).await?;
    
    let body = res.text().await?;

    Ok(body)
}