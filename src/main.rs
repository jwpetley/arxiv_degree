mod scrape;

use scrape::{get_authors, daily_authors};

fn main() {
    let authors = get_authors("J.+W.+Petley");

    let daily_authors = daily_authors();
    println!("{:?}", authors);
    println!("{:?}", daily_authors);
}
