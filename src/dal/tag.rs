use juniper::FieldResult;

use crate::Ctx;
use crate::resolvers::tag::{Tag, TagInput};

impl Tag {
    pub async fn get_tag(ctx: &Ctx, id: &str) -> FieldResult<Tag> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, name FROM tag WHERE id=$1").await?;
        let id_i32 = id.parse::<i32>()?;
        let row = db.query_one(&stmt, &[&id_i32]).await?;
        Ok(Tag{
            id: row.get::<&str,i32>("id").to_string(),
            name: row.get("name"),
            article_ids: vec![]
        })
    }

    pub async fn get_all_tags(ctx: &Ctx) -> FieldResult<Vec<Tag>> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, name FROM tag").await?;
        let rows = db.query(&stmt, &[]).await?;

        Ok(rows.iter().map(|r| {
            Tag{
                id: r.get::<&str,i32>("id").to_string(),
                name: r.get("name"),
                article_ids: vec![]
            }
        }).collect())
    }

    pub async fn get_article_tags(ctx: &Ctx, article_id: &str) -> FieldResult<Vec<Tag>> {
        let db = ctx.db_pool.get().await?;
        let article_id_i32 = article_id.parse::<i32>()?;
        let stmt = db.prepare("SELECT id, name FROM tag, article_tag WHERE tag.id = article_tag.tag_id AND article_tag.article_id = $1").await?;
        let rows = db.query(&stmt, &[&article_id_i32]).await?;


        Ok(rows.iter().map(|r| {
            Tag{
                id: r.get::<&str,i32>("id").to_string(),
                name: r.get("name"),
                article_ids: vec![]
            }
        }).collect())
    }

    pub async fn create_tag(ctx: &Ctx, input: TagInput) -> FieldResult<Tag> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("INSERT INTO tag(name) VALUES ($1) RETURNING id").await?;
        let row = db.query_one(&stmt, &[&input.name]).await?;

        Ok(Tag{
            id: row.get::<&str,i32>("id").to_string(),
            name: input.name,
            article_ids: vec![]
        })
    }
}
