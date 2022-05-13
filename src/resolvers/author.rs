use juniper::graphql_object;

use crate::Article;

pub struct Author {
    pub id: String,
    pub name: String,
    pub articles_ids: Vec<String>,
}

#[graphql_object]
#[graphql(description="Person who writes things")]
impl Author {
    pub fn id(&self) -> &str { return &self.id; }
    pub fn name(&self) -> &str { return &self.name; }

    pub fn articles(&self) -> Vec<Article> {
        println!("articles resolving!");
        vec![]
    }
}
