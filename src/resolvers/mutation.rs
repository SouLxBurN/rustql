use juniper::{graphql_object, FieldResult};

use crate::Ctx;
use crate::resolvers::tag::Tag;
use crate::resolvers::tag::TagInput;

pub struct Mutation;
#[graphql_object(Context = Ctx)]
impl Mutation {
    pub async fn createTag(ctx: &Ctx, input: TagInput) -> FieldResult<Tag> {
        let stmt = ctx.db.prepare("INSERT INTO tag(name) VALUES ($1) RETURNING id").await?;
        let row = ctx.db.query_one(&stmt, &[&input.name]).await?;

        Ok(Tag{
            id: row.get::<&str,i32>("id").to_string(),
            name: input.name,
            article_ids: vec![]
        })
    }
}

