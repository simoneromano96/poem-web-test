use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    extensions::Tracing,
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Enum, FieldResult, Interface, Object, Request,
    Response, Schema,
};
use async_graphql_poem::GraphQL;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html, Json},
    EndpointExt, IntoResponse, Route, Server,
};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use tracing::{debug, error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

// GraphQL API
type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> String {
        String::from("pong")
    }
}

#[handler]
async fn graphql_handler(schema: Data<&GraphQlSchema>, req: Json<Request>) -> Json<Response> {
    debug!("GraphQL ping handler");
    Json(schema.execute(req.0).await)
}

// REST API
struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn index(&self) -> PlainText<String> {
        debug!("REST ping handler");
        PlainText(String::from("pong"))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    //* TRACING
    // Create a jaeger exporter pipeline for a `trace_demo` service.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("trace_demo")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error initializing Jaeger exporter");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not registrer default subscriber");
    //* END TRACING

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    let api_service = OpenApiService::new(Api)
        .title("Hello World")
        .server("http://localhost:3000/api");

    let ui = api_service.swagger_ui("http://localhost:3000");

    let listener = TcpListener::bind("127.0.0.1:3000");

    poem::Server::new(listener)
        .await?
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/graphql", post(graphql_handler))
                .nest("/", ui)
                .data(schema.clone()),
        )
        .await
}
