use axum::body::{Body, Bytes};
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use futures::stream;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct RandomParams {
    #[serde(default = "random_default_output")]
    output: Output,
}

#[derive(Deserialize, IntoParams)]
pub struct RandomStreamParams {
    #[serde(default = "random_stream_default_size")]
    #[param(minimum = 0, maximum = 65535, default = 1024)]
    size: u16,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Output {
    U32,
    U64,
    U128,
    F32,
    F64,
}

fn random_default_output() -> Output {
    Output::U64
}

fn random_stream_default_size() -> u16 {
    1024
}

#[utoipa::path(
    get,
    path = "/api/random",
    params(
        RandomParams
    ),
    responses(
        (status = 200, body = String, example = json!(12345))
    )
)]
pub async fn get_random(Query(params): Query<RandomParams>) -> Response {
    match params.output {
        Output::U32 => rand::random::<u32>().to_string(),
        Output::U64 => rand::random::<u64>().to_string(),
        Output::U128 => rand::random::<u128>().to_string(),
        Output::F32 => rand::random::<f32>().to_string(),
        Output::F64 => rand::random::<f64>().to_string(),
    }
    .into_response()
}

struct StreamState {
    state: usize,
    random_generator: StdRng,
}

#[utoipa::path(
    get,
    path = "/api/random/stream",
    params(
        RandomStreamParams
    ),
    responses(
        (status = 200, body = String, example = json!(12345))
    )
)]
pub async fn get_random_stream(Query(params): Query<RandomStreamParams>) -> Response {
    let size = params.size as usize;

    let state = StreamState {
        state: 0,
        random_generator: StdRng::from_entropy(),
    };

    let stream = stream::unfold(state, move |mut state| async move {
        if state.state <= (size - 1) {
            let random = state.random_generator.next_u64();
            let bytes = Bytes::copy_from_slice(&random.to_be_bytes());
            state.state = state.state + bytes.len();

            Some((Ok::<_, String>(bytes), state))
        } else {
            None
        }
    });

    let body = Body::from_stream(stream);

    body.into_response()
}
