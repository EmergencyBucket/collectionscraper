use amiquip::{
    AmqpValue, Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions,
};

use crate::utils::read_env_var;

pub fn get_connection() -> Result<(), amiquip::Error> {
    let mut connection = Connection::insecure_open(&read_env_var("RABBIT_URL")).unwrap();

    let channel = connection.open_channel(None).unwrap();

    let mut arguments = FieldTable::new();
    arguments.insert("x-max-priority".to_owned(), AmqpValue::ShortInt(5));
    let options = QueueDeclareOptions {
        arguments,
        durable: true,
        ..QueueDeclareOptions::default()
    };

    let queue = channel.queue_declare("task_queue", options).unwrap();

    let consumer = queue.consume(ConsumerOptions::default()).unwrap();

    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
