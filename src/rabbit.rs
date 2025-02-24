use amiquip::{
    AmqpValue, Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions,
};

use crate::utils::read_env_var;

pub fn get_connection() -> Connection {
    let mut connection = Connection::insecure_open(&read_env_var("RABBIT_URL")).unwrap();

    return connection;
}
