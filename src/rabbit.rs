use lapin::{Connection, ConnectionProperties};

use crate::utils::read_env_var;

pub async fn get_connection() -> Connection {
    let connection =
        Connection::connect(&read_env_var("RABBIT_URL"), ConnectionProperties::default())
            .await
            .unwrap();

    return connection;
}
