#![allow(unused)]

#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello_path/Jack").await?.print().await?;

    Ok(())
}
