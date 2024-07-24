use crate::paper::{Paper};

#[derive(Clone, Debug)]
pub struct Author {
    pub name: String,
    pub search_link: String,
    //papers: Vec<Paper>,
    //co_authors: Vec<Author>
}

impl Author {
    
    pub fn get_surname(&self) -> String {
        let surname: Vec<&str> = self.name.split(" ").collect();
        let len = surname.len();
        surname[len-1].to_string()
    }

}