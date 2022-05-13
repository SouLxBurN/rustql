extern crate juniper;

mod resolvers;
use self::resolvers::{Article, Query};
use self::resolvers::enums::Language;

use std::env;
use juniper::*;
use warp::Filter;

// #[derive(GraphQLInputObject)]
// #[graphql(description="A humanoid creature in the Star Wars universe")]
// struct NewHumanInput {
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

pub struct Ctx(Language);
impl juniper::Context for Ctx {}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "warp_server");

    let log = warp::log("warp_server");

    let state = warp::any().map(move || Ctx(Language::EN));
    let graphql_filter = juniper_warp::make_graphql_filter(
        Schema::new(
            Query,
            EmptyMutation::new(),
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
    .await
}
