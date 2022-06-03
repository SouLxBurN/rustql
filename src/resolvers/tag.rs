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

#[graphql_object(Context = Ctx)]
#[graphql(description="Article classification")]
impl Tag {
    pub fn id(&self) -> &str { return &self.id; }
    pub fn name(&self) -> &str { return &self.name; }
    pub async fn articles(&self, ctx: &Ctx) -> FieldResult<Vec<Article>> {
        Article::get_tag_articles(ctx, &self.id).await
    }
}

impl Tag {
    pub async fn get_tag(ctx: &Ctx, id: &str) -> FieldResult<Tag> {
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

    pub async fn get_article_tags(ctx: &Ctx, article_id: &str) -> FieldResult<Vec<Tag>> {
        let article_id_i32 = article_id.parse::<i32>()?;
        let stmt = ctx.db.prepare("SELECT id, name FROM tag, article_tag WHERE tag.id = article_tag.tag_id AND article_tag.article_id = $1").await?;
        let rows = ctx.db.query(&stmt, &[&article_id_i32]).await?;

        Ok(rows.iter().map(|r| {
            Tag{
                id: r.get::<&str,i32>("id").to_string(),
                name: r.get("name"),
                article_ids: vec![]
            }
        }).collect())
    }

    pub async fn create_tag(ctx: &Ctx, input: TagInput) -> FieldResult<Tag> {
        let stmt = ctx.db.prepare("INSERT INTO tag(name) VALUES ($1) RETURNING id").await?;
        let row = ctx.db.query_one(&stmt, &[&input.name]).await?;

        Ok(Tag{
            id: row.get::<&str,i32>("id").to_string(),
            name: input.name,
            article_ids: vec![]
        })
    }
}
