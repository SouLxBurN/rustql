use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::resolvers::{Article, Language};

pub struct Query;
#[graphql_object(Context = Ctx)]
impl Query {
    pub fn apiVersion() -> &str {
        "1.0"
    }

   pub fn article(ctx: &Ctx, id: String) -> FieldResult<Article> {
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
