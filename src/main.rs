use clap::Parser;
use webbrowser;

#[derive(Parser)]
/// An object containing the parsed command line arguments
struct SearchQuery {
    #[clap(required(true))]
    /// The query to be searched using Google or the specified search engine
    query: String,

    #[clap(short, long)]
    /// Use Wikipedia
    wikipedia: bool,

    #[clap(short, long)]
    /// Use YouTube
    youtube: bool,

    #[clap(short, long)]
    /// Use Reddit
    reddit: bool,

    #[clap(short, long)]
    /// Use Twitter
    twitter: bool,

    #[clap(short, long)]
    /// Use GitHub
    github: bool,

    #[clap(short, long)]
    /// Use StackOverflow
    stackoverflow: bool,

    #[clap(short, long)]
    /// Use Crates.io
    cratesio: bool,
}

impl SearchQuery {
    /// Perform the specified search
    fn search_and_open(&self) {
        let engine = if self.wikipedia {
            "wikipedia"
        } else if self.youtube {
            "youtube"
        } else if self.reddit {
            "reddit"
        } else if self.twitter {
            "twitter"
        } else if self.github {
            "github"
        } else if self.stackoverflow {
            "stackoverflow"
        } else if self.cratesio {
            "cratesio"
        } else {
            "google"
        };

        let url = if engine != "wikipedia" && engine != "cratesio" {
            format!("https://www.{}.com/search?q={}", engine, self.query)
        } else if engine == "wikipedia" {
            format!("https://en.wikipedia.org/w/index.php?search={}", self.query)
        } else {
            format!("https://crates.io/search?q={}", self.query)
        };

        webbrowser::open(&url).unwrap_or_else(|_| {
            println!("Failed to open {}", url);
        });
    }
}

fn main() {
    let query = SearchQuery::parse();
    query.search_and_open();
}
