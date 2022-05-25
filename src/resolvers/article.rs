use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::resolvers::enums::Language;
use crate::resolvers::author::Author;
use crate::resolvers::tag::Tag;

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

impl Article {
    pub async fn get_article(ctx: &Ctx, id: String) -> FieldResult<Article> {
        let rows = ctx.db.query("SELECT $1::TEXT", &[&"Hello Stream"]).await?;
        let value: &str = rows[0].get(0);

        println!("{value}");

        Ok(Article{
            id,
            title: String::from("My Favorite Child"),
            body: String::from("This should be a long body, but its not"),
            language: Language::EN,
            author_ids: vec![String::from("1234")],
            tag_ids: vec![String::from("4321")]
        })
    }
}
