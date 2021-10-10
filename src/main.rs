use poem::{listener::TcpListener, Route};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use tracing::{debug, error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        #[oai(name = "name", in = "query")] name: Option<String>,
    ) -> PlainText<String> {
        debug!("Test, {:?}", name);
        match name {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

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

    let listener = TcpListener::bind("127.0.0.1:3000");

    let api_service = OpenApiService::new(Api)
        .title("Hello World")
        .server("http://localhost:3000/api");

    let ui = api_service.swagger_ui("http://localhost:3000");

    poem::Server::new(listener)
        .await?
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
