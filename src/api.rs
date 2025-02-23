use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use reqwest::Url;
use nestify::nest;

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

nest! {
    #[derive(Serialize, Deserialize, Debug)]*
    pub struct BungieResponse {
        pub Response: pub struct Response {
            pub profileCollectibles: pub struct ProfileCollectibles {
                pub data: pub struct Data {
                    pub collectibles: HashMap<u32, pub struct Collectible {
                        pub state: u8
                    }>
                }
            }
        }
    }
}

pub async fn make_bungie_request(path: &str) -> BungieResponse {
    let url = Url::parse_with_params(
        format!("https://www.bungie.net/Platform{}", path).as_str(),
        &[("random", rand::random::<u32>().to_string())],
    )
    .unwrap();

    let addr = generate_address();

    let mut client_builder = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .http3_prior_knowledge();

    if std::env::var("ENVIRONMENT").unwrap_or("production".to_owned()) != "development" {
        client_builder = client_builder.local_address(addr);
    }

    let client = client_builder
        .build()
        .unwrap();

    let res = client
        .get(url)
        .header("X-API-Key", BUNGIE_KEY)
        .send()
        .await
        .unwrap()
        .json::<BungieResponse>()
        .await
        .unwrap();
    res
}

pub async fn get_collections(membership_type: u8, membership_id: u64) {
    let req = make_bungie_request(&format!(
        "/Destiny2/{}/Profile/{}/?components=800",
        membership_type, membership_id
    )).await;

    //gjson::get(&req, "Response.profileCollectibles.data.collectibles");
}