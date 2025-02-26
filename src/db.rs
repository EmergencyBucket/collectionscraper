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
}

#[derive(Row, Serialize)]
pub struct UsersRow {
    pub timestamp: i64,
    pub membershipId: i64,
    pub membershipType: i8,
    pub bungieName: String,
    pub lastPlayed: i64,
    pub profileData: String,
    pub collections: Vec<i64>,
    pub emblems: Vec<u32>,
}

pub async fn push_data(data: Vec<UsersRow>) {
    let client = get_client();

    let mut inserter = client
        .inserter::<UsersRow>("users_v3")
        .unwrap()
        .with_timeouts(Some(Duration::from_secs(5)), Some(Duration::from_secs(20)))
        .with_max_bytes(50_000_000)
        .with_max_rows(750_000)
        .with_period(Some(Duration::from_secs(15)))
        .with_timeouts(
            Some(Duration::from_secs(1_000_000)),
            Some(Duration::from_secs(1_000_000)),
        );

    for row in data {
        if row.emblems.len() == 0 {
            continue;
        }

        inserter.write(&row).unwrap();
    }

    let stats = inserter.commit().await.unwrap();

    if stats.rows > 0 {
        println!(
            "{} bytes, {} rows, {} transactions have been inserted",
            stats.bytes, stats.rows, stats.transactions,
        );
    }

    inserter.end().await.unwrap();
}
