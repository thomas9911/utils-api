mod common;

use common::setup;

#[tokio::test]
async fn graphql_prettier_works() -> anyhow::Result<()> {
    let input = "{allSongs{results{id}}}";
    let expected = r#"{
  allSongs {
    results {
      id
    }
  }
}
"#;
    let ctx = setup().await?;
    let resp = ctx.post("api/graphql/prettier", input).await?;
    let graphql = resp.text().await?;

    assert_eq!(graphql, expected);
    Ok(())
}

#[tokio::test]
async fn graphql_minifier_works() -> anyhow::Result<()> {
    let input = r#"{
  allSongs {
    results {
      id
    }
  }
}
"#;
    let expected = "{allSongs{results{id}}}";

    let ctx = setup().await?;
    let resp = ctx.post("api/graphql/minifier", input).await?;
    let graphql = resp.text().await?;

    assert_eq!(graphql, expected);
    Ok(())
}
