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
