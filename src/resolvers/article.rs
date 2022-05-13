use juniper::graphql_object;

use crate::Language;
use crate::resolvers::{Author, Tag};

pub struct Article {
    pub id: String,
    pub title: String,
    pub body: String,
    pub language: Language,
    pub author_ids: Vec<String>,
    pub tag_ids: Vec<String>
}

#[graphql_object]
#[graphql(description="Its an Article")]
impl Article {
    pub fn id(&self) -> &str { return &self.id; }
    pub fn title(&self) -> &str { return &self.title; }
    pub fn body(&self) -> &str { return &self.body; }
    pub fn language(&self) -> &Language { return &self.language; }

    pub fn authors(&self) -> Vec<Author> {
        vec![Author{
                id: String::from("4321"),
                name:String::from("SouLxBurN"),
                articles_ids: vec![]
            }]
    }
    pub fn tags(&self) -> Vec<Tag> {
        vec![Tag{
                id: String::from("3214"),
                name: String::from("children"),
                article_ids: vec![]
            }]
    }
}

