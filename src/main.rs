use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

mod random;
mod uuid;

#[derive(OpenApi)]
#[openapi(
    paths(random::get_random, uuid::get_uuid,),
    components(schemas(random::Output, uuid::Format, uuid::Version))
)]
struct ApiDoc;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    3000
}

impl Config {
    fn new() -> Config {
        let builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("UTILS_API").ignore_empty(true));

        builder.build().unwrap().try_deserialize().unwrap()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new();

    let router = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/api-docs"))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/uuid", get(uuid::get_uuid))
        .route("/api/random", get(random::get_random));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
