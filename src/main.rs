mod functions;
mod graphql;
mod models;
mod routes;

use async_graphql::{Request, Response};
use functions::{get_db::get_db, init_tracing::init_tracing};
use graphql::create_schema;
use poem::middleware::Cors;
use poem::{
	get, handler,
	listener::TcpListener,
	post,
	web::{Data, Json},
	EndpointExt, Response as PoemResponse, Route,
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

#[handler]
async fn spec_handler(spec: Data<&String>) -> PoemResponse {
	let spec = spec.0.to_string();
	PoemResponse::builder()
		.content_type("application/json")
		.body(spec)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "poem=debug");
	}
	// Init tracing
	init_tracing();

	// DB first initialization (this is not strictly necessary, however if the db is
	// not accessible at the start of the application, the application shouldn't
	// start)
	get_db().await;

	// GraphQL initialization
	let schema = create_schema();

	// REST initialization
	let api_service = OpenApiService::new(TodoAPI)
		.title("Hello World")
		.server("http://localhost:3000/api");

	// SwaggerUI
	// let ui = api_service.swagger_ui("http://localhost:3000");
	let spec = api_service.spec();

	// HTTP Server
	let listener = TcpListener::bind("127.0.0.1:3000");

	let cors = Cors::new();

	poem::Server::new(listener)
		.await?
		.run(
			Route::new()
				.nest("/api", api_service)
				.nest("/graphql", post(graphql_handler))
				.nest("/spec", get(spec_handler))
				// .nest("/", ui)
				.data(spec)
				.data(schema)
				.with(cors),
		)
		.await
}
