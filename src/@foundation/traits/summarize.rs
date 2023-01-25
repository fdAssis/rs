pub trait Summarize {
    fn author(&self) -> String;
    fn summary(&self) {
        println!("Author: {}", self.author());
    }
}
pub struct Post {
    author: String,
    text: String,
    title: String,
}
pub struct Article {
    author: String,
    title: String,
    num_pages: u32,
}

impl Summarize for Post {
    fn author(&self) -> String {
        self.author.clone()
    }
}
