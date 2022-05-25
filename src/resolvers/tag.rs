use juniper::{graphql_object, GraphQLInputObject, FieldResult};

use crate::Ctx;
use crate::resolvers::article::Article;

pub struct Tag {
    pub id: String,
    pub name: String,
    pub article_ids: Vec<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description="Tag input parameters")]
pub struct TagInput {
    pub name: String
}

#[graphql_object]
#[graphql(description="Article classification")]
impl Tag {
    fn id(&self) -> &str { return &self.id; }
    fn name(&self) -> &str { return &self.name; }

    fn articles(&self) -> Vec<Article> {
        vec![]
    }
}

impl Tag {
    pub async fn get_tag(ctx: &Ctx, id: String) -> FieldResult<Tag> {
        let stmt = ctx.db.prepare("SELECT id, name FROM tag WHERE id=$1").await?;
        let id_i32 = id.parse::<i32>()?;
        let row = ctx.db.query_one(&stmt, &[&id_i32]).await?;
        Ok(Tag{
            id: row.get::<&str,i32>("id").to_string(),
            name: row.get("name"),
            article_ids: vec![]
        })
    }

    pub async fn get_all_tags(ctx: &Ctx) -> FieldResult<Vec<Tag>> {
        let stmt = ctx.db.prepare("SELECT id, name FROM tag").await?;
        let rows = ctx.db.query(&stmt, &[]).await?;

        Ok(rows.iter().map(|r| {
            Tag{
                id: r.get::<&str,i32>("id").to_string(),
                name: r.get("name"),
                article_ids: vec![]
            }
        }).collect())
    }
}
