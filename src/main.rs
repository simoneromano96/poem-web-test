mod functions;
mod graphql;
mod models;
mod routes;

use async_graphql::{
    extensions::Tracing, EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use functions::{get_db::get_db, init_tracing::init_tracing};
use graphql::todo::QueryRoot;
use poem::{
    handler,
    listener::TcpListener,
    post,
    web::{Data, Json},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use routes::todo::TodoAPI;
use tracing::debug;

use crate::graphql::GraphQlSchema;

#[handler]
async fn graphql_handler(schema: Data<&GraphQlSchema>, req: Json<Request>) -> Json<Response> {
    debug!("GraphQL ping handler");
    Json(schema.execute(req.0).await)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // Init tracing
    init_tracing();

    // DB first initialization
    get_db().await;

    // GraphQL initialization
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    // REST initialization
    let api_service = OpenApiService::new(TodoAPI)
        .title("Hello World")
        .server("http://localhost:3000/api");

    // SwaggerUI
    let ui = api_service.swagger_ui("http://localhost:3000");

    // HTTP Server
    let listener = TcpListener::bind("127.0.0.1:3000");

    poem::Server::new(listener)
        .await?
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/graphql", post(graphql_handler))
                .nest("/", ui)
                .data(schema),
        )
        .await
}
