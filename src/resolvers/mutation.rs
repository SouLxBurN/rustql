use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::dal::article::ArticleDAL;
use crate::resolvers::article::Article;
use crate::resolvers::article::ArticleInput;
use crate::resolvers::author::Author;
use crate::resolvers::author::AuthorInput;
use crate::resolvers::tag::Tag;
use crate::resolvers::tag::TagInput;

pub struct Mutation;
#[graphql_object(Context = Ctx)]
impl Mutation {
    /// Create a new article
    pub async fn createArticle(ctx: &Ctx, input: ArticleInput) -> FieldResult<Article> {
        ArticleDAL::create_article(ctx, input).await
    }
    /// Create a new author
    pub async fn createAuthor(ctx: &Ctx, input: AuthorInput) -> FieldResult<Author> {
        Author::create_author(ctx, input).await
    }
    /// Create a new tag
    pub async fn createTag(ctx: &Ctx, input: TagInput) -> FieldResult<Tag> {
        Tag::create_tag(ctx, input).await
    }
}

