use std::time::Duration;

use clickhouse::Client;

use clickhouse::Row;
use serde::Serialize;

use crate::utils::read_env_var;

pub fn get_client() -> Client {
    Client::default()
        .with_url(read_env_var("CLICKHOUSE_URL"))
        .with_user(read_env_var("CLICKHOUSE_USER"))
        .with_password(read_env_var("CLICKHOUSE_PASSWORD"))
        .with_option("async_insert", "1")
        .with_option("wait_for_async_insert", "0")
}

#[derive(Row, Serialize)]
pub struct UsersRow {
    pub timestamp: i64,
    pub membershipId: i64,
    pub membershipType: i8,
    pub bungieName: String,
    pub lastPlayed: i64,
    pub profileData: String,
    pub collections: Vec<u32>,
    pub emblems: Vec<u32>,
}

pub async fn push_data(data: Vec<UsersRow>) {
    let client = get_client();

    let mut insert = client
        .insert::<UsersRow>("users_v3")
        .unwrap()
        .with_timeouts(
            Some(Duration::from_secs(1_000_000)),
            Some(Duration::from_secs(1_000_000)),
        );

    for row in data {
        if row.collections.len() == 0 {
            continue;
        }

        insert.write(&row).await.unwrap();
    }

    insert.end().await.unwrap();
}

pub async fn get_users(limit: u64, offset: u64) -> Vec<u64> {
    let client = get_client();

    let query = format!(
        "SELECT user_id FROM old_emblem_data LIMIT {} OFFSET {}",
        limit, offset
    );

    let result = client.query(&query).fetch_all().await.unwrap();

    result
}

pub async fn get_users_count() -> u64 {
    let client = get_client();

    let query = "SELECT count() FROM old_emblem_data FINAL";

    let result = client.query(&query).fetch_all().await.unwrap();

    let count: u64 = result[0];

    count
}
