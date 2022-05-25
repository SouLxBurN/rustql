extern crate juniper;

mod resolvers;
use crate::resolvers::query::Query;

use std::env;
use std::sync::Arc;
use juniper::*;
use tokio_postgres::{Error, Client, NoTls};
use warp::Filter;

use self::resolvers::mutation::Mutation;

pub struct Ctx {
    db: Arc<Client>
}

impl Ctx {
    fn new(c: Arc<Client>) -> Self {
        Self {
            db: c
        }
    }
}
impl juniper::Context for Ctx {}

type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "warp_server");
    let log = warp::log("warp_server");

    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=example", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let rc_client = Arc::new(client);

    let state = warp::any().map(move || {Ctx::new(rc_client.clone())});
    let graphql_filter = juniper_warp::make_graphql_filter(
        Schema::new(
            Query,
            Mutation,
            EmptySubscription::new()),
        state.boxed()
    );

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
