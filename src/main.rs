use axum::routing::get;
use axum::Router;

mod random;
mod uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/uuid", get(uuid::get_uuid))
        .route("/api/random", get(random::get_random));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
