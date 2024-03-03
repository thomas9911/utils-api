use escargot::CargoRun;
use reqwest::Response;
use std::marker::PhantomData;
use std::process::Child;
use tokio::sync::OnceCell;

static ONCE: OnceCell<CargoRun> = OnceCell::const_new();

const HOST: &str = "http://localhost";

#[derive(Debug)]
struct TestContext<'a> {
    child_process: Child,
    port: u16,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TestContext<'a> {
    fn new(child_process: Child, port: u16) -> Self {
        TestContext {
            child_process,
            port,
            _phantom: PhantomData::default(),
        }
    }

    async fn get(&self, path: &str) -> reqwest::Result<Response> {
        reqwest::get(format!("{HOST}:{}/{path}", self.port)).await
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

async fn setup<'a>() -> anyhow::Result<TestContext<'a>> {
    let new_port = portpicker::pick_unused_port().expect("No ports free");

    let res = ONCE.get_or_try_init(build_server_bin).await?;

    let child = res
        .command()
        .env("UTILS_API_PORT", new_port.to_string())
        .spawn()
        .unwrap();

    Ok(TestContext::new(child, new_port))
}

#[tokio::test]
async fn random_default_returns_u64() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/random").await?;
    let _: u64 = resp.text().await?.parse()?;

    let resp = ctx.get("api/random").await?;
    assert!(resp.text().await?.parse::<u32>().is_err());
    Ok(())
}

#[tokio::test]
async fn random_returns_u32() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/random?output=u32").await?;
    let _: u64 = resp.text().await?.parse()?;

    let resp = ctx.get("api/random?output=u32").await?;
    assert!(resp.text().await?.parse::<u16>().is_err());
    Ok(())
}

#[tokio::test]
async fn random_returns_u64() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/random?output=u64").await?;
    let _: u64 = resp.text().await?.parse()?;

    let resp = ctx.get("api/random?output=u64").await?;
    assert!(resp.text().await?.parse::<u32>().is_err());
    Ok(())
}

#[tokio::test]
async fn random_returns_u128() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/random?output=u128").await?;
    let _: u128 = resp.text().await?.parse()?;

    let resp = ctx.get("api/random?output=u128").await?;
    assert!(resp.text().await?.parse::<u64>().is_err());
    Ok(())
}
