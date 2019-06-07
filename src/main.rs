extern crate structopt;

use serde::{Serialize, Deserialize};

use structopt::StructOpt;

// Struct to store our command line arguments
#[derive(StructOpt)]
struct Cli {
    /// The number of top posts to retrieve
    num: usize,
    #[structopt(short = "p", long = "--progress")]
    progress: bool,
    #[structopt(short = "b", long = "by")]
    by: bool,
    #[structopt(short = "s", long = "score")]
    score: bool,
    #[structopt(short = "u", long = "url")]
    url: bool
}


//Structure of each item.
#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: i32,
    by: String,
    descendants: i32,
    score: i32,
    url: String,
    title: String,
}

fn main() {

    let args = Cli::from_args();
    let num = args.num;

    //TODO: Don't use unwrap(). Handle the Result.
    let mut top_500_ids: Vec<i32> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").unwrap().json().unwrap();
    top_500_ids.resize(num, 0);
    let top_30_ids = top_500_ids;

    let mut top_items: Vec<Item> = Vec::new();

    //This is so as to display the progress bar. See the indicatif crate
    let mut pb = indicatif::ProgressBar::new(num as u64);

    if !args.progress {
        pb = indicatif::ProgressBar::hidden();
    }

    for (_i, id) in top_30_ids.iter().enumerate() {
        //TODO: Move all the ugly hardcoded API urls to somewhere else? Maybe a constants struct?
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        let item: Item = reqwest::get(&url).unwrap().json().unwrap();
        top_items.push(item);
        pb.inc(1);
    }
    pb.finish();

    // TODO: Make the output format configurable somehow
    for item in top_items {
        println!("{}", item.title);
        if args.score {
            print!("Score: {} ", item.score);
        }
        if args.url {
            print!("Url: {} ", item.url);
        }
        if args.by {
            print!("By: {}", item.by);
        }
        println!("\n");
    }

}