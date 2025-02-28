use std::{time::SystemTime, vec};

use api::get_collections;
use db::{get_users, get_users_count, push_data};
use futures::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
};
use rabbit::get_connection;

pub mod api;
pub mod db;
pub mod rabbit;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let user_count = get_users_count().await - 1000;

    let connection = get_connection().await;

    let channel = connection.create_channel().await.unwrap();

    channel
        .basic_qos(1000, BasicQosOptions::default())
        .await
        .unwrap();

    let mut arguments = FieldTable::default();
    arguments.insert(
        "x-max-priority".to_owned().into(),
        lapin::types::AMQPValue::ShortInt(5),
    );
    let options = QueueDeclareOptions {
        durable: true,
        ..QueueDeclareOptions::default()
    };

    let queue = channel
        .queue_declare("task_queue", options, arguments)
        .await
        .unwrap();

    println!("Declared queue");

    let mut consumer_options = BasicConsumeOptions::default();

    consumer_options.no_local = true;

    let mut consumer = channel
        .basic_consume(
            "task_queue",
            "my_consumer",
            consumer_options,
            FieldTable::default(),
        )
        .await
        .unwrap();

    println!("Waiting for messages. Press Ctrl-C to exit.");

    let mut lv: Vec<u64> = vec![];

    let mut count = 0;

    loop {
        if queue.message_count() > 10000 {
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                delivery.ack(BasicAckOptions::default()).await.expect("ack");

                println!("Received message #{}", count);
                count += 1;

                let body = String::from_utf8_lossy(&delivery.data);

                let mut tem: Vec<u64> = serde_json::from_str(&body).unwrap();

                lv.append(&mut tem);

                if lv.len() >= 1000 {
                    process_message(lv.iter().map(|x| (None as Option<i8>, x.clone())).collect()).await;
                    lv = vec![];
                    break;
                }
            }
        }

        if queue.message_count() < 10000 {
            println!("Using clickhouse data!");

            let users = get_users(1000, rand::random_range(0..user_count)).await;

            process_message(users.iter().map(|x| (Some(x.0), x.1 - 4611686018000000000)).collect()).await;
        }
    }
}

async fn process_message(ids: Vec<(Option<i8>, u64)>) {
    let start = SystemTime::now();

    println!(
        "Getting ids starting at {} going to {}",
        ids[0].1,
        ids[ids.len() - 1].1
    );

    let mut reqs = vec![];

    for (i, x) in ids.iter().enumerate() {
        let req = get_collections(x.1.clone(), x.0, i as u32);
        reqs.push(req);
    }

    println!("Making {} requests", reqs.len() * 2);

    // Proccess in 100 request chunks

    let c = trpl::join_all(reqs).await;

    push_data(c).await;

    let end = SystemTime::now();

    println!("Finished in {:?}", end.duration_since(start).unwrap());
}
