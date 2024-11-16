use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use minify_js::{minify, Session, TopLevelMode};

const PRETTY_EXAMPLE: &str = "function a() {
  return 1
}";

const MINIFIED_EXAMPLE: &str = "var a=(()=>1)";

#[utoipa::path(
    post,
    path = "/api/javascript/minifier",
    operation_id = "post_javascript_minifier",
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
        let session = Session::new();
        let mut out = Vec::new();
        if let Some(Ok(minified)) =
            minify(&session, TopLevelMode::Global, data.as_bytes(), &mut out)
                .ok()
                .map(|_| String::from_utf8(out))
        {
            minified.into_response()
        } else {
            data.into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "invalid body").into_response()
    }
}
