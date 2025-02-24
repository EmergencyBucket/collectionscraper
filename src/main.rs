use std::time::SystemTime;

use amiquip::{AmqpValue, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};
use api::get_collections;
use db::push_data;
use rabbit::get_connection;

pub mod api;
pub mod db;
pub mod rabbit;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut connection = get_connection();

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
                process_message(body.to_string()).await;
                //println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close().unwrap();

    Ok(())
}

async fn process_message(message: String) {
    let start = SystemTime::now();

    let deserialized: Vec<String> = serde_json::from_str(&message).unwrap();

    let ids: Vec<u64> = deserialized.iter().map(|x| x.parse::<u64>().unwrap()).collect();

    println!("## Getting ids starting at {} going to {}", &ids[0], &ids[ids.len() - 1]);

    let mut reqs = vec![];

    for i in &ids {
        let req = get_collections(*i);
        reqs.push(req);
    }

    println!("Making {} requests", reqs.len()*2);

    let c = trpl::join_all(reqs).await;

    push_data(c).await;

    let end = SystemTime::now();

    println!("## Finished in {:?}", end.duration_since(start).unwrap());
}