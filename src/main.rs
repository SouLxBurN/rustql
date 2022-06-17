extern crate juniper;

mod resolvers;
mod dal;
use crate::resolvers::query::Query;

use dataloader::non_cached::Loader;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime, Pool};
use juniper::*;
use std::env;
use std::ops::DerefMut;
use tokio_postgres::{Error, NoTls};
use warp::Filter;

use self::dal::article::{ArticleLoader, ArticleDAL};
use self::resolvers::article::Article;
use self::resolvers::mutation::Mutation;

pub struct Ctx {
    pub db_pool: Pool,
    pub article_loader: Loader<i32, Article, ArticleLoader>
}

impl Ctx {
    fn new(p: Pool, al: Loader<i32, Article, ArticleLoader>)-> Self {
        Self {
            db_pool: p,
            article_loader: al,
        }
    }
}
impl juniper::Context for Ctx {}

type Schema<'a> = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("db/");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "warp_server");
    let log = warp::log("warp_server");

    let mut cfg = Config::new();
    cfg.host = Some(String::from("localhost"));
    cfg.dbname = Some(String::from("rustql"));
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("example"));
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    if let Ok(mut obj) = pool.get().await {
        let client = obj.deref_mut().deref_mut();
        match embedded::migrations::runner().run_async(client).await {
            Ok(report) => println!("{:?}", report),
            Err(e) => panic!("Migrations Failed: {e}"),
        }
    }

    let art_dal = ArticleDAL::new(pool.clone());
    let state = warp::any().map(move || Ctx::new(
        pool.clone(),
        Loader::new(ArticleLoader::new(art_dal.clone())).with_max_batch_size(5)
    ));

    let schema = Schema::new(Query, Mutation, EmptySubscription::new());
    let graphql_filter = juniper_warp::make_graphql_filter(schema, state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path::end())
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql")
            .and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;
    Ok(())
}
