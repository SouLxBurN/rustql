use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::BatchFn;
use deadpool_postgres::Pool;
use juniper::FieldResult;

use crate::Ctx;
use crate::resolvers::article::{Article, ArticleInput};

pub struct ArticleLoader {
    dal: ArticleDAL
}

impl ArticleLoader {
    pub fn new(dal: ArticleDAL) -> Self { Self { dal } }
}

#[async_trait]
impl BatchFn<i32, Article> for ArticleLoader {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Article> {
        if let Ok(article_map) = self.dal.get_batch_articles(keys).await {
            return article_map;
        }
        println!("Failed to get articles");
        HashMap::new()
    }
}

#[derive(Clone)]
pub struct ArticleDAL {
    db_pool: Pool
}

impl ArticleDAL {
    pub fn new(db_pool: Pool) -> Self {
        Self{ db_pool }
    }

    pub async fn get_batch_articles(&self, keys: &[i32]) -> FieldResult<HashMap<i32, Article>> {
        let db = self.db_pool.get().await?;

        let mut params = keys[0].to_string();
        for idx in 1..keys.len() {
            params += &format!(",{}", keys[idx]).to_owned();
        }

        let stmt = db.prepare(
            format!("SELECT id, title, body, language, author_id FROM article WHERE id IN ({})", params)
        .as_str()).await?;
        let rows = db.query(&stmt, &[]).await?;

        Ok(rows.iter().map(|r| {
            let id = r.get::<&str, i32>("id");
            (id, Article{
                id: id.to_string(),
                title: r.get("title"),
                body: r.get("body"),
                language: r.get::<&str, String>("language").into(),
                author_id: r.get::<&str, i32>("author_id").to_string(),
            })
        }).collect())
    }

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
        let id_i32 = id.parse::<i32>()?;
        Ok(ctx.article_loader.load(id_i32).await)
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
