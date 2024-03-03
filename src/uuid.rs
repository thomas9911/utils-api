use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct UuidParams {
    #[serde(default = "uuid_default_format")]
    format: Format,
    #[serde(default = "uuid_default_version")]
    version: Version,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    Braced,
    Hyphenated,
    Simple,
    Urn,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Version {
    V4,
    V7,
}

fn uuid_default_format() -> Format {
    Format::Hyphenated
}

fn uuid_default_version() -> Version {
    Version::V4
}

#[utoipa::path(
    get,
    path = "/api/uuid",
    params(
        UuidParams
    ),
    responses(
        (status = 200, body = String, example = json!("4ffa0c85-4031-4293-8515-3396ac5fe716"))
    )
)]
pub async fn get_uuid(Query(params): Query<UuidParams>) -> Response {
    let uuid = match params.version {
        Version::V4 => uuid::Uuid::new_v4(),
        Version::V7 => uuid::Uuid::now_v7(),
    };

    match params.format {
        Format::Braced => uuid.as_braced().to_string(),
        Format::Hyphenated => uuid.as_hyphenated().to_string(),
        Format::Simple => uuid.as_simple().to_string(),
        Format::Urn => uuid.as_urn().to_string(),
    }
    .into_response()
}
