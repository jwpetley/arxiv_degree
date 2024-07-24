use crate::author::{Author};

#[derive(Clone, Debug)]
pub struct Paper {
    pub name: String,
    pub id: String,
    pub first_author: Author,
    pub authors: Vec<Author>

}