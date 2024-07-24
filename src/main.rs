mod scrape;
mod paper;
mod author;
mod search;

use scrape::{get_authors, daily_authors, daily_papers};
use author::{Author};

fn main() {
    let authors = get_authors("J.+W.+Petley");

    let daily_authors = daily_authors();
    let daily_papers = daily_papers();
    //println!("{:?}", authors);
    //println!("{:?}", authors[0].get_surname())
    //println!("{:?}", daily_authors);
}
