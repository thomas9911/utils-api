use axum::routing::get;
use axum::Router;

mod random;
mod uuid;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(random::get_random, uuid::get_uuid,),
    components(schemas(random::Output, uuid::Format, uuid::Version))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/api-docs"))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/uuid", get(uuid::get_uuid))
        .route("/api/random", get(random::get_random));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
