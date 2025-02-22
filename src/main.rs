use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let proxy = reqwest::Proxy::all("http://0.0.0.0:51080")?;

    let client = reqwest::Client::builder().proxy(proxy).build()?;

    let res = client.get("http://ipv6.ip.sb").send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
