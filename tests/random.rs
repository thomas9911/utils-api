mod common;

use common::setup;

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
