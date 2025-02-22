use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = IpAddr::from([
        0x2a01,
        0x4f9,
        0x3051,
        0x4a65,
        rand::random::<u16>(),
        rand::random::<u16>(),
        rand::random::<u16>(),
        rand::random::<u16>(),
    ]);

    let client = reqwest::Client::builder().local_address(addr).build()?;

    let res = client.get("https://api64.ipify.org?format=text").send().await?.text().await?;
    println!("{res:#?}");

    Ok(())
}
