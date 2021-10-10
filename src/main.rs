mod models;

use async_graphql::{
    extensions::Tracing, EmptyMutation, EmptySubscription, Error as GraphQLError, Object, Request,
    Response, Result as GraphQLResult, Schema,
};
use poem::{
    handler,
    listener::TcpListener,
    post,
    web::{Data, Json},
    EndpointExt, Route,
};
use poem_openapi::{
    payload::{Json as OpenAPIJson, PlainText},
    ApiResponse, OpenApi, OpenApiService, Tags,
};
use sea_orm::EntityTrait;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;
use tracing::debug;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use models::prelude::Todo;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn get_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let db = Database::connect("postgres://poem_test:poem_test@localhost/poem_test")
            .await
            .expect("Connection to the database failed");
        db
    })
    .await
}

async fn get_todos() -> Result<Vec<models::todo::TodoDAO>, DbErr> {
    let db = get_db().await;
    let todos: Vec<models::todo::Model> = Todo::find().all(db).await?;
    Ok(todos.into_iter().map(|todo| todo.into()).collect())
}

// GraphQL API
type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> String {
        String::from("pong")
    }

    async fn todos(&self) -> GraphQLResult<Vec<models::todo::TodoDAO>> {
        debug!("GraphQL todos handler");
        match get_todos().await {
            Ok(todos) => Ok(todos),
            Err(error) => Err(GraphQLError::from(format!("{}", error))),
        }
    }
}

#[handler]
async fn graphql_handler(schema: Data<&GraphQlSchema>, req: Json<Request>) -> Json<Response> {
    debug!("GraphQL ping handler");
    Json(schema.execute(req.0).await)
}

// REST API

#[derive(Tags)]
enum ApiTags {
    /// Operations about todo
    Todo,
}

#[derive(ApiResponse)]
enum GetTodosResponse {
    /// Got all todos.
    #[oai(status = 200)]
    Ok(OpenAPIJson<Vec<models::todo::TodoDAO>>),
    /// Had database error.
    #[oai(status = 500)]
    DbError,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn index(&self) -> PlainText<String> {
        debug!("REST ping handler");
        PlainText(String::from("pong"))
    }
    #[oai(path = "/todo", method = "get", tag = "ApiTags::Todo")]
    async fn todos(&self) -> GetTodosResponse {
        debug!("REST todos handler");
        match get_todos().await {
            Ok(todos) => GetTodosResponse::Ok(OpenAPIJson(todos)),
            Err(_error) => GetTodosResponse::DbError,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    //* TRACING
    // Log compatibility
    LogTracer::init().expect("Log compatibility could not be initialized");

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

    // DB first initialization
    get_db().await;

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
