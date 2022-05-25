use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::resolvers::article::Article;
use crate::resolvers::tag::Tag;

pub struct Query;
#[graphql_object(Context = Ctx)]
impl Query {

    /// Retrieve a single article by ID
    pub async fn article(ctx: &Ctx, id: String) -> FieldResult<Article> {
        Article::get_article(ctx,id).await
    }

    /// Retrieve a single tag by ID
    pub async fn tag(ctx: &Ctx, id: String) -> Option<Tag> {
        match Tag::get_tag(ctx, id).await {
            Ok(tag) => Some(tag),
            Err(_e) => None // Probably should log this error
        }
    }

    /// Retrieve ALL tags in the database
    pub async fn tags(ctx: &Ctx) -> FieldResult<Vec<Tag>> {
        Tag::get_all_tags(ctx).await
    }
}
