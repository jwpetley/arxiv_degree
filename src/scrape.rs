use crate::{author::Author, paper::{self, Paper}};

use std::iter::zip;

use reqwest;
use scraper::{self, element_ref::Select};

fn author_search_request(author: &str) -> String {
    let response = reqwest::blocking::get(
        format!("https://arxiv.org/search/?query={author}&searchtype=all&source=header"),
    )
    .unwrap()
    .text()
    .unwrap();

    response
}

fn authors_from_response(response: &str) -> Vec<Author> {
    let document = scraper::Html::parse_document(&response);

    let author_selector = scraper::Selector::parse(".authors>a")
        .unwrap();

    
    let authors_search = document.select(&author_selector)
        .map(|x| x.html()
                            .split("\"")
                            .nth(1)
                            .unwrap()
                            .to_string());

    let authors_name = document.select(&author_selector)
        .map(|x| x.inner_html());

    let mut authors: Vec<Author> = Vec::new();
    
    for (author, search) in zip(authors_name, authors_search) {
        let tmp: Author = Author {
            name: author,
            search_link: search,
        };
        authors.push(tmp);
    }
    
    authors
}

pub fn get_authors(author: &str) -> Vec<Author> {
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

fn papers_from_response(response: &str) -> Vec<Paper> {
    let document = scraper::Html::parse_document(&response);

    let paper_selector = scraper::Selector::parse("dd")
        .unwrap();

    let link_selector = scraper::Selector::parse("dt")
        .unwrap();

    let author_selector = scraper::Selector::parse(".list-authors>a")
        .unwrap();
    
    let title_selector = scraper::Selector::parse(".list-title>span")
        .unwrap();

    let papers = document.select(&paper_selector).map(|x| x.html());

    let ids = document.select(&link_selector)
        .map(|x| x.text().nth(3)
                .unwrap().to_string()
                .split_whitespace().collect());

    let vec_ids: Vec<String> = ids.collect();


    let mut papers_vec: Vec<Paper> = Vec::new();

    for (i, paper) in papers.enumerate() {
        let html = scraper::Html::parse_fragment(&paper);
        let authors = html.select(&author_selector);
        
        let title = html.select(&title_selector).map(|x| x.inner_html());
        

        let authors_vec = authors_from_select(authors);
        
        let id = &vec_ids[i];

        let tmp: Paper = Paper { name: (title.last().unwrap()), 
            id: (id.to_string()), 
            first_author: (authors_vec[0].clone()), 
            authors: (authors_vec) };

        papers_vec.push(tmp)

        }

    papers_vec


}



pub fn daily_papers() -> Vec<Paper> {
    let response = daily_arxiv_request();

    let papers = papers_from_response(&response);

    papers
}

fn authors_from_select(authors: scraper::html::Select) -> Vec<Author> {
    let names = authors.clone().map(|x| x.inner_html());
    let searches = authors.map(|x| x.html()
        .split("\"")
        .nth(1)
        .unwrap()
        .to_string());


    let mut authors: Vec<Author> = Vec::new();
    for (name, search) in zip(names, searches) {
        let tmp: Author = Author { name: (name), 
            search_link: (search) };
        authors.push(tmp);
    }

    authors
}
