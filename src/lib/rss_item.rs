use std::fmt;
pub struct rss_item {
    pub title: String,
    pub url: String,
    pub description: String,
}

impl fmt::Display for rss_item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "title: {}", self.title);
        writeln!(f, "\turl: {}", self.url);
        writeln!(f, "\tdescription: {}", self.description); 
        Ok(())
    }
}