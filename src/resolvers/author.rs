use juniper::{graphql_object, GraphQLInputObject, FieldResult};

use crate::Ctx;
use crate::resolvers::article::Article;

pub struct Author {
    pub id: String,
    pub name: String,
    pub article_ids: Vec<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description="Author input parameters")]
pub struct AuthorInput {
    pub name: String
}

#[graphql_object(Context = Ctx)]
#[graphql(description="Person who writes things")]
impl Author {
    pub fn id(&self) -> &str { return &self.id; }
    pub fn name(&self) -> &str { return &self.name; }

    pub async fn articles(&self, ctx: &Ctx) -> FieldResult<Vec<Article>> {
        Article::get_author_articles(ctx, &self.id).await
    }
}

