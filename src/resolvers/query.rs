use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::dal::article::ArticleDAL;
use crate::resolvers::article::Article;
use crate::resolvers::author::Author;
use crate::resolvers::tag::Tag;

pub struct Query;
#[graphql_object(Context = Ctx)]
impl Query {
    /// Retrieve a single article by ID
    pub async fn article(ctx: &Ctx, id: String) -> Option<Article> {
        ArticleDAL::get_article(ctx, &id).await.ok()
    }
    /// Retrieves a all articles
    pub async fn articles(ctx: &Ctx) -> FieldResult<Vec<Article>> {
        ArticleDAL::get_all_articles(ctx).await
    }
    /// Retrieve a single tag by ID
    pub async fn tag(ctx: &Ctx, id: String) -> Option<Tag> {
        Tag::get_tag(ctx, &id).await.ok()
    }
    /// Retrieve ALL tags in the database
    pub async fn tags(ctx: &Ctx) -> FieldResult<Vec<Tag>> {
        Tag::get_all_tags(ctx).await
    }
    /// Retrieve a single author by ID
    pub async fn author(ctx: &Ctx, id: String) -> Option<Author> {
        Author::get_author(ctx, &id).await.ok()
    }
    /// Retrieve ALL tags in the database
    pub async fn authors(ctx: &Ctx) -> FieldResult<Vec<Author>> {
        Author::get_all_authors(ctx).await
    }
}
