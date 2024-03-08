use escargot::CargoRun;
use reqwest::{Body, Client, Response};
use std::marker::PhantomData;
use std::process::Child;
use tokio::sync::OnceCell;

static ONCE: OnceCell<CargoRun> = OnceCell::const_new();

const HOST: &str = "http://localhost";

#[derive(Debug)]
pub struct TestContext<'a> {
    child_process: Child,
    port: u16,
    client: Client,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TestContext<'a> {
    pub fn new(child_process: Child, port: u16) -> Self {
        TestContext {
            child_process,
            port,
            client: Client::new(),
            _phantom: PhantomData::default(),
        }
    }

    pub async fn get(&self, path: &str) -> reqwest::Result<Response> {
        self.client
            .get(format!("{HOST}:{}/{path}", self.port))
            .send()
            .await
    }

    pub async fn post<T: Into<Body>>(&self, path: &str, body: T) -> reqwest::Result<Response> {
        self.client
            .post(format!("{HOST}:{}/{path}", self.port))
            .body(body)
            .send()
            .await
    }
}

impl Drop for TestContext<'_> {
    fn drop(&mut self) {
        self.child_process.kill().unwrap();
    }
}

async fn build_server_bin() -> anyhow::Result<CargoRun> {
    let cargo_run = escargot::CargoBuild::new()
        .bin(env!("CARGO_PKG_NAME"))
        .run()
        .unwrap();

    Ok(cargo_run)
}

pub async fn setup<'a>() -> anyhow::Result<TestContext<'a>> {
    let new_port = portpicker::pick_unused_port().expect("No ports free");

    let res = ONCE.get_or_try_init(build_server_bin).await?;

    let child = res
        .command()
        .env("UTILS_API_PORT", new_port.to_string())
        .spawn()
        .unwrap();

    Ok(TestContext::new(child, new_port))
}
