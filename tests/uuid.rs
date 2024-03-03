mod common;

use common::setup;

#[tokio::test]
async fn uuid_default_returns_hypenated_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid").await?;
    let uuid = resp.text().await?;

    assert_eq!(uuid.len(), 36);
    assert_eq!(uuid.chars().filter(|ch| ch == &'-').count(), 4);
    Ok(())
}

#[tokio::test]
async fn uuid_returns_hypenated_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid?format=hypenated").await?;
    let uuid = resp.text().await?;

    assert_eq!(uuid.len(), 36);
    assert_eq!(uuid.chars().filter(|ch| ch == &'-').count(), 4);
    Ok(())
}

#[tokio::test]
async fn uuid_returns_simple_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid?format=simple").await?;
    let uuid = resp.text().await?;

    assert_eq!(uuid.len(), 32);
    Ok(())
}

#[tokio::test]
async fn uuid_returns_urn_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid?format=urn").await?;
    let uuid = resp.text().await?;

    assert_eq!(uuid.len(), 45);
    Ok(())
}

#[tokio::test]
async fn uuid_returns_braced_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid?format=braced").await?;
    let uuid = resp.text().await?;

    assert_eq!(uuid.len(), 38);
    assert_eq!(
        uuid.chars().filter(|ch| ch == &'{' || ch == &'}').count(),
        2
    );
    Ok(())
}

#[tokio::test]
async fn uuid_returns_hypenated_v7_uuid() -> anyhow::Result<()> {
    let ctx = setup().await?;
    let resp = ctx.get("api/uuid?version=v7&format=hyphenated").await?;
    let uuid1 = resp.text().await?;
    let resp = ctx.get("api/uuid?version=v7&format=hyphenated").await?;
    let uuid2 = resp.text().await?;

    assert_eq!(uuid1.len(), 36);
    assert_eq!(uuid2.len(), 36);

    // first part is the same for v7
    let prefix1 = uuid1.split_once('-').map(|(prefix, _)| prefix).unwrap();
    let prefix2 = uuid2.split_once('-').map(|(prefix, _)| prefix).unwrap();

    assert_eq!(prefix1, prefix2);

    Ok(())
}
