use juniper::{graphql_object, GraphQLInputObject, FieldResult};

use crate::Ctx;
use crate::dal::article::ArticleDAL;
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
        ArticleDAL::get_tag_articles(ctx, &self.id).await
    }
}

