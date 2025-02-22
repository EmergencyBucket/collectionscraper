use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let proxy = reqwest::Proxy::all("http://0.0.0.0:51080")?;

    let client = reqwest::Client::builder().proxy(proxy).build()?;

    let res = client.get("http://httpbin.org/ip").send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
