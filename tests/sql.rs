pub mod common;

use common::setup;

#[tokio::test]
async fn sql_prettier_works() -> anyhow::Result<()> {
    let input = "select id from users";
    let expected = "SELECT\n  id\nFROM\n  users";
    let ctx = setup().await?;
    let resp = ctx.post("api/sql/prettier", input).await?;
    let sql = resp.text().await?;

    assert_eq!(sql, expected);
    Ok(())
}
