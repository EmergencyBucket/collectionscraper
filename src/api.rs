/// No changes can be made with this API key so it can be public
const BUNGIE_KEY: &'static str = "529cac5f9e3a482b86b931f1f75f2331";

/// Generates a random ipv6 address in the subnet  
/// This subnet is specific to the eBucket server ```2a01:4f9:3051:4a65::/64```
fn generate_address() -> std::net::IpAddr {
    std::net::IpAddr::from([
        0x2a01,
        0x4f9,
        0x3051,
        0x4a65,
        rand::random::<u16>(),
        rand::random::<u16>(),
        rand::random::<u16>(),
        rand::random::<u16>(),
    ])
}

pub async fn make_request(url: &str) -> String {
    let addr = generate_address();

    let client = reqwest::Client::builder()
        .local_address(addr)
        .build()
        .unwrap();

    let res = client.get(url).send().await.unwrap().text().await.unwrap();
    res
}

pub async fn make_bungie_request(path: &str) -> String {
    let url = format!(
        "https://www.bungie.net/Platform{}?random={}",
        path,
        rand::random::<u32>()
    );
    let addr = generate_address();

    let client = reqwest::Client::builder()
        .local_address(addr)
        .build()
        .unwrap();

    let res = client
        .get(&url)
        .header("X-API-Key", BUNGIE_KEY)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}
