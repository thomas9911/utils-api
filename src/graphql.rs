// use axum::body::Body;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use graphql_minify::minify;
use graphql_parser::parse_query;

const PRETTY_EXAMPLE: &str = "{
  allSongs {
    results {
      id
    }
  }
}";

const MINIFIED_EXAMPLE: &str = "{allSongs{results{id}}}";

#[utoipa::path(
    post,
    path = "/api/graphql/prettier",
    responses(
        (status = 200, body = String, content_type = "text/plain", example = json!(PRETTY_EXAMPLE))
    ),
    request_body(
        content = String,
        content_type = "text/plain",
        example = json!(MINIFIED_EXAMPLE)
    )
)]
pub async fn post_prettier(req: Request) -> Response {
    if let Ok(data) = String::from_request(req, &()).await {
        if let Ok(ast) = parse_query::<&str>(&data) {
            ast.to_string().into_response()
        } else {
            data.into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "invalid body").into_response()
    }
}

#[utoipa::path(
    post,
    path = "/api/graphql/minifier",
    responses(
        (status = 200, body = String, content_type = "text/plain", example = json!(MINIFIED_EXAMPLE))
    ),
    request_body(
        content = String,
        content_type = "text/plain",
        example = json!(PRETTY_EXAMPLE)
    )
)]
pub async fn post_minifier(req: Request) -> Response {
    if let Ok(data) = String::from_request(req, &()).await {
        if let Ok(minfied) = minify(&data) {
            minfied.into_response()
        } else {
            data.into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "invalid body").into_response()
    }
}
