# RustQL

GraphQL server exploration using [Juniper](https://github.com/graphql-rust/juniper)

## Dependencies

- [juniper](https://github.com/graphql-rust/juniper) GraphQL library
- [warp](https://github.com/seanmonstar/warp) Web server framework
- [tokio](https://github.com/tokio-rs/tokio) async/await runtime
- [tokio-postgres](https://github.com/sfackler/rust-postgres) async postgres database driver.
- [refinery](https://github.com/rust-db/refinery) Database migrations

## Running Locally

1. Run `docker-compose up` to start the dependencies.
2. Run `cargo run` to startup of the server.
3. Navigate to `localhost:8080` to for graphiql.
4. Profit!
