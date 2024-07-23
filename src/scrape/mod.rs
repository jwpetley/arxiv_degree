use reqwest;
use scraper;

fn author_search_request(author: &str) -> String {
    let response = reqwest::blocking::get(
        format!("https://arxiv.org/search/?query={author}&searchtype=all&source=header"),
    )
    .unwrap()
    .text()
    .unwrap();

    response
}

fn authors_from_response(response: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(&response);

    let author_selector = scraper::Selector::parse(".authors>a")
        .unwrap();

    let authors = document.select(&author_selector)
        .map(|x| x.inner_html());

    authors.collect()
}

pub fn get_authors(author: &str) -> Vec<String> {
    let response = &author_search_request(author);

    let authors = authors_from_response(response);

    authors
}

fn daily_arxiv_request() -> String {
    let response = reqwest::blocking::get(
        "https://arxiv.org/list/astro-ph/new"
    )
    .unwrap()
    .text()
    .unwrap();

    response
}

fn daily_authors_from_response(response: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(&response);

    let author_selector = scraper::Selector::parse(".list-authors>a")
        .unwrap();

    let authors = document.select(&author_selector)
        .map(|x| x.inner_html());

    authors.collect()
}

pub fn daily_authors() -> Vec<String> {
    let response = daily_arxiv_request();

    let authors = daily_authors_from_response(&response);

    authors
}

