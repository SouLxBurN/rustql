use juniper::graphql_object;

use crate::resolvers::Article;

pub struct Tag {
    pub id: String,
    pub name: String,
    pub article_ids: Vec<String>,
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
