extern crate juniper;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod resolvers;
mod dal;
use crate::resolvers::query::Query;

use juniper::*;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls};
use warp::Filter;

use self::resolvers::mutation::Mutation;

pub struct Ctx {
    pub db: Arc<Client>,
}

impl Ctx {
    fn new(c: Arc<Client>) -> Self {
        Self { db: c }
    }
}
impl juniper::Context for Ctx {}

type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("db/");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "warp_server");
    let log = warp::log("warp_server");

    let (mut client, connection) = tokio_postgres::connect(
        "dbname=rustql host=localhost user=postgres password=example",
        NoTls,
    )
    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    match embedded::migrations::runner().run_async(&mut client).await {
        Ok(report) => println!("{:?}", report),
        Err(e) => panic!("Migrations Failed: {e}"),
    }

    let rc_client = Arc::new(client);
    let state = warp::any().map(move || Ctx::new(rc_client.clone()));

    let schema = Schema::new(Query, Mutation, EmptySubscription::new());
    println!("{}", schema.as_schema_language());
    let graphql_filter = juniper_warp::make_graphql_filter(schema, state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path::end())
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;
    Ok(())
}
