use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UuidParams {
    #[serde(default = "uuid_default_format")]
    format: Format,
    #[serde(default = "uuid_default_version")]
    version: Version,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Braced,
    Hyphenated,
    Simple,
    Urn,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
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
