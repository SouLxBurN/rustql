use juniper::FieldResult;

use crate::Ctx;
use crate::resolvers::article::{Article, ArticleInput};

impl Article {
    pub async fn get_all_articles(ctx: &Ctx) -> FieldResult<Vec<Article>> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, title, body, language, author_id FROM article").await?;
        let rows = db.query(&stmt, &[]).await?;

        Ok(rows.iter().map(|r| {
            Article{
                id: r.get::<&str, i32>("id").to_string(),
                title: r.get("title"),
                body: r.get("body"),
                language: r.get::<&str, String>("language").into(),
                author_id: r.get::<&str, i32>("author_id").to_string(),
            }
        }).collect())
    }

    pub async fn get_tag_articles(ctx: &Ctx, tag_id: &str) -> FieldResult<Vec<Article>> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, title, body, language, author_id FROM article a, article_tag at WHERE a.id = at.article_id AND at.tag_id = $1").await?;
        let tag_id_i32 = tag_id.parse::<i32>()?;
        let rows = db.query(&stmt, &[&tag_id_i32]).await?;

        Ok(rows.iter().map(|r| {
            Article{
                id: r.get::<&str, i32>("id").to_string(),
                title: r.get("title"),
                body: r.get("body"),
                language: r.get::<&str, String>("language").into(),
                author_id: r.get::<&str, i32>("author_id").to_string(),
            }
        }).collect())
    }

    pub async fn get_author_articles(ctx: &Ctx, author_id: &str) -> FieldResult<Vec<Article>> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, title, body, language, author_id FROM article WHERE author_id = $1").await?;
        let author_id_i32 = author_id.parse::<i32>()?;
        let rows = db.query(&stmt, &[&author_id_i32]).await?;

        Ok(rows.iter().map(|r| {
            Article{
                id: r.get::<&str, i32>("id").to_string(),
                title: r.get("title"),
                body: r.get("body"),
                language: r.get::<&str, String>("language").into(),
                author_id: r.get::<&str, i32>("author_id").to_string(),
            }
        }).collect())
    }

    pub async fn get_article(ctx: &Ctx, id: &str) -> FieldResult<Article> {
        let db = ctx.db_pool.get().await?;
        let stmt = db.prepare("SELECT id, title, body, language, author_id FROM article WHERE id=$1").await?;
        let id_i32 = id.parse::<i32>()?;
        let row = db.query_one(&stmt, &[&id_i32]).await?;

        Ok(Article{
            id: row.get::<&str,i32>("id").to_string(),
            title: row.get("title"),
            body: row.get("body"),
            language: row.get::<&str, String>("language").into(),
            author_id: row.get::<&str,i32>("author_id").to_string(),
        })
    }

    pub async fn create_article(ctx: &Ctx, input: ArticleInput) -> FieldResult<Article> {
        // TODO Transactify this please
        let db = ctx.db_pool.get().await?;
        let author_id_i32 = input.author_id.parse::<i32>()?;
        let article_stmt = db.prepare("INSERT INTO article(title, body, language, author_id) VALUES ($1, $2, $3, $4) RETURNING *").await?;
        let article_row = db.query_one(&article_stmt, &[&input.title, &input.body, &input.language.to_string(), &author_id_i32]).await?;

        if let Ok(article_id) = article_row.try_get::<&str, i32>("id") {
            for t_id in input.tag_ids {
                let t_id_i32 = t_id.parse::<i32>()?;
                let tag_stmt = db.prepare("INSERT INTO article_tag(article_id, tag_id) VALUES ($1, $2)").await?;
                let _result = db.execute(&tag_stmt, &[&article_id, &t_id_i32]).await?;
            }
        };

        Ok(Article{
            id: article_row.get::<&str,i32>("id").to_string(),
            title: article_row.get("title"),
            body: article_row.get("body"),
            language: article_row.get::<&str, String>("language").into(),
            author_id: article_row.get::<&str,i32>("author_id").to_string(),
        })
    }
}
