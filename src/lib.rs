use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use tokio::signal::ctrl_c;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

mod graphql;
mod random;
mod uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        random::get_random,
        uuid::get_uuid,
        graphql::post_prettier,
        graphql::post_minifier
    ),
    components(schemas(random::Output, uuid::Format, uuid::Version))
)]
pub struct ApiDoc;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    3000
}

impl Config {
    pub fn new() -> Config {
        let builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("UTILS_API").ignore_empty(true));

        builder.build().unwrap().try_deserialize().unwrap()
    }
}

fn api_router() -> Router {
    Router::new()
        .route("/uuid", get(uuid::get_uuid))
        .route("/random", get(random::get_random))
        .route("/graphql/prettier", post(graphql::post_prettier))
        .route("/graphql/minifier", post(graphql::post_minifier))
}

pub async fn main() -> anyhow::Result<()> {
    let config = Config::new();

    let router = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/api-docs"))
        .nest("/api", api_router())
        .route("/", get(|| async { Redirect::permanent("/api-docs") }));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port)).await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(signal())
        .await?;

    Ok(())
}

async fn signal() {
    ctrl_c().await.expect("failed to listen for event")
}
