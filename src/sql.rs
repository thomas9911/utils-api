use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlformat::{FormatOptions, QueryParams};

const PRETTY_EXAMPLE: &str = "SELECT\n  id\nFROM\n  users";
const MINIFIED_EXAMPLE: &str = "select id from users";

#[utoipa::path(
    post,
    path = "/api/sql/prettier",
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
        let mut options = FormatOptions::default();
        options.uppercase = true;
        sqlformat::format(&data, &QueryParams::None, options).into_response()
    } else {
        (StatusCode::BAD_REQUEST, "invalid body").into_response()
    }
}
