use amiquip::Connection;

use crate::utils::read_env_var;

pub fn get_connection() -> Connection {
    let connection = Connection::insecure_open(&read_env_var("RABBIT_URL")).unwrap();

    return connection;
}
