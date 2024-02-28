use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RandomParams {
    #[serde(default = "random_default_output")]
    output: Output,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Output {
    U32,
    U64,
    U128,
}

fn random_default_output() -> Output {
    Output::U64
}

pub async fn get_random(Query(params): Query<RandomParams>) -> Response {
    match params.output {
        Output::U32 => rand::random::<u32>().to_string(),
        Output::U64 => rand::random::<u64>().to_string(),
        Output::U128 => rand::random::<u128>().to_string(),
    }
    .into_response()
}
