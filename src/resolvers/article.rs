use juniper::{graphql_object, FieldResult, GraphQLInputObject};

use crate::Ctx;
use crate::resolvers::enums::Language;
use crate::resolvers::author::Author;
use crate::resolvers::tag::Tag;

pub struct Article {
    pub id: String,
    pub title: String,
    pub body: String,
    pub language: Language,
    pub author_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description="Article input parameters")]
pub struct ArticleInput {
    pub title: String,
    pub body: String,
    pub language: Language,
    pub author_id: String,
    pub tag_ids: Vec<String>,
}

#[graphql_object(Context = Ctx)]
#[graphql(description="Its an Article")]
impl Article {
    pub fn id(&self) -> &str { return &self.id; }
    pub fn title(&self) -> &str { return &self.title; }
    pub fn body(&self) -> &str { return &self.body; }
    pub fn language(&self) -> &Language { return &self.language; }
    pub async fn author(&self, ctx: &Ctx) -> FieldResult<Author> {
        Author::get_author(ctx, &self.author_id).await
    }
    pub async fn tags(&self, ctx: &Ctx) -> FieldResult<Vec<Tag>> {
        Tag::get_article_tags(ctx, &self.id).await
    }
}

