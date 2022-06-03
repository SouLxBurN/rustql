use juniper::FieldResult;

use crate::Ctx;
use crate::resolvers::author::{Author, AuthorInput};

impl Author {
    pub async fn get_author(ctx: &Ctx, id: &str) -> FieldResult<Author> {
        let stmt = ctx.db.prepare("SELECT id, name FROM author WHERE id=$1").await?;
        let id_i32 = id.parse::<i32>()?;
        let row = ctx.db.query_one(&stmt, &[&id_i32]).await?;
        Ok(Author{
            id: row.get::<&str,i32>("id").to_string(),
            name: row.get("name"),
            article_ids: vec![]
        })
    }

    pub async fn get_all_authors(ctx: &Ctx) -> FieldResult<Vec<Author>> {
        let stmt = ctx.db.prepare("SELECT id, name FROM author").await?;
        let rows = ctx.db.query(&stmt, &[]).await?;

        Ok(rows.iter().map(|r| {
            Author{
                id: r.get::<&str,i32>("id").to_string(),
                name: r.get("name"),
                article_ids: vec![]
            }
        }).collect())
    }

    pub async fn create_author(ctx: &Ctx, input: AuthorInput) -> FieldResult<Author> {
        let stmt = ctx.db.prepare("INSERT INTO author(name) VALUES ($1) RETURNING id").await?;
        let row = ctx.db.query_one(&stmt, &[&input.name]).await?;

        Ok(Author{
            id: row.get::<&str,i32>("id").to_string(),
            name: input.name,
            article_ids: vec![]
        })
    }
}
