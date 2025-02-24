use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use nestify::nest;
use reqwest::{Response, Url};

use crate::db::UsersRow;

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

pub async fn make_bungie_request(path: String) -> Option<Response> {
    let url = Url::parse_with_params(
        format!("https://www.bungie.net/Platform{}", path).as_str(),
        &[("random", rand::random::<u32>().to_string())],
    )
    .unwrap();

    let addr = generate_address();

    let mut client_builder = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .pool_idle_timeout(Duration::from_secs(5))
        .http3_prior_knowledge()
        .use_rustls_tls();

    if std::env::var("ENVIRONMENT").unwrap_or("production".to_owned()) != "development" {
        client_builder = client_builder.local_address(addr);
    }

    let client = client_builder.build().unwrap();

    let res = client.get(url).header("X-API-Key", BUNGIE_KEY).send().await;

    if res.is_err() {
        return None;
    } else {
        return Some(res.unwrap());
    }
}

pub async fn get_membership_details(membership_id: u64) -> (u8, String) {
    nest! {
        #[derive(Serialize, Deserialize)]*
        struct GetLinkedProfiles {
            Response: struct BungieResponse {
                profiles: Vec<struct Profile {
                    membershipType: u8,
                    displayName: String
                }>
            }
        }
    }

    let req = make_bungie_request(format!(
        "/Destiny2/-1/Profile/{}/LinkedProfiles/",
        membership_id
    ));

    let res = req.await;

    if res.is_none() {
        return (0, "".to_owned());
    }

    let ja = res.unwrap().json::<GetLinkedProfiles>().await;

    if ja.is_err() {
        return (0, "".to_owned());
    }

    let json: GetLinkedProfiles = ja.unwrap();

    if json.Response.profiles.len() == 0 {
        return (0, "".to_owned());
    }

    (
        json.Response.profiles[0].membershipType,
        json.Response.profiles[0].displayName.clone(),
    )
}

#[derive(PartialEq)]
enum CollectibleState {
    None,
    NotAcquired,
    Obscured,
    Invisible,
    CannotAffordMaterialRequirements,
    InventorySpaceUnavailable,
    UniquenessViolation,
    PurchaseDisabled,
}

/// Decodes the collectible state from a u8  
/// returns a vector of CollectibleState
fn decode_state(state: u8) -> Vec<CollectibleState> {
    const NONE: u8 = 0;
    const NOT_ACQUIRED: u8 = 1;
    const OBSCURED: u8 = 2;
    const INVISIBLE: u8 = 4;
    const CANNOT_AFFORD_MATERIAL_REQUIREMENTS: u8 = 8;
    const INVENTORY_SPACE_UNAVAILABLE: u8 = 16;
    const UNIQUENESS_VIOLATION: u8 = 32;
    const PURCHASE_DISABLED: u8 = 64;

    let mut states: Vec<CollectibleState> = vec![];

    if state & NONE == NONE {
        states.push(CollectibleState::None);
    }

    if state & NOT_ACQUIRED == NOT_ACQUIRED {
        states.push(CollectibleState::NotAcquired);
    }

    if state & OBSCURED == OBSCURED {
        states.push(CollectibleState::Obscured);
    }

    if state & INVISIBLE == INVISIBLE {
        states.push(CollectibleState::Invisible);
    }

    if state & CANNOT_AFFORD_MATERIAL_REQUIREMENTS == CANNOT_AFFORD_MATERIAL_REQUIREMENTS {
        states.push(CollectibleState::CannotAffordMaterialRequirements);
    }

    if state & INVENTORY_SPACE_UNAVAILABLE == INVENTORY_SPACE_UNAVAILABLE {
        states.push(CollectibleState::InventorySpaceUnavailable);
    }

    if state & UNIQUENESS_VIOLATION == UNIQUENESS_VIOLATION {
        states.push(CollectibleState::UniquenessViolation);
    }

    if state & PURCHASE_DISABLED == PURCHASE_DISABLED {
        states.push(CollectibleState::PurchaseDisabled);
    }

    return states;
}

pub async fn get_collections(membership_id: u64) -> UsersRow {
    let default = UsersRow {
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        membershipId: 0,
        membershipType: 0,
        bungieName: "".to_string(),
        lastPlayed: 0,
        profileData: "".to_owned(),
        collections: vec![],
        emblems: vec![],
    };

    let offset: u64 = 4611686018000000000;

    let id = membership_id + offset;

    // First we need to get the membershipType
    let membership_details = get_membership_details(id).await;

    let membership_type = membership_details.0;

    let name = membership_details.1;

    if membership_type == 0 {
        return default;
    }

    nest! {
        #[derive(Serialize, Deserialize)]*
        struct GetProfile {
            Response: Option<struct Profile {
                profileCollectibles: struct Collectibles {
                    data: Option<struct CollectibleData {
                        collectibles: HashMap<u32, struct Collectible {
                            state: u8
                        }>
                    }>
                }
            }>
        }
    }

    let ra = make_bungie_request(format!(
        "/Destiny2/{}/Profile/{}/?components=800",
        membership_type, id
    ))
    .await;

    if ra.is_none() {
        return default;
    }

    let ja = ra.unwrap().json::<GetProfile>().await;

    if ja.is_err() {
        return default;
    }

    let req: GetProfile = ja.unwrap();

    if req.Response.is_none() {
        return default;
    }

    let profile = req.Response.unwrap().profileCollectibles.data;

    if profile.is_none() {
        return default;
    }

    let collectibles = profile.unwrap().collectibles;

    let mut emblems: Vec<u32> = vec![];

    for (key, value) in collectibles {
        let states = decode_state(value.state);

        if !states.contains(&CollectibleState::NotAcquired) && states.len() > 0 {
            emblems.push(key);
        }
    }

    UsersRow {
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        membershipId: id as i64,
        membershipType: membership_type as i8,
        bungieName: name,
        lastPlayed: 0,
        profileData: "".to_owned(),
        collections: vec![],
        emblems: emblems,
    }
}
