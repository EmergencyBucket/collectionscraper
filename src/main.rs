use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let client = reqwest::Client::builder().proxy(reqwest::Proxy::all("http://127.0.0.1:51080").unwrap()).build().unwrap();

    let res = client.get("http://httpbin.org/ip").send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
