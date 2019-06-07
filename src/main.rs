extern crate clap;

extern crate structopt;

use std::io::Read;
use std::fmt;

use serde::{Serialize, Deserialize};

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
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
    let mut top_500_ids: Vec<i32> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").unwrap().json().unwrap();
    top_500_ids.resize(num, 0);
    let top_30_ids = top_500_ids;

    let mut top_items: Vec<Item> = Vec::new();

    let mut pb = indicatif::ProgressBar::new(num as u64);

    if !args.progress {
        pb = indicatif::ProgressBar::hidden();
    }

    for (i, id) in top_30_ids.iter().enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        let item: Item = reqwest::get(&url).unwrap().json().unwrap();
        top_items.push(item);
        pb.inc(1);
    }
    pb.finish();

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