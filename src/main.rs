use std::{time::SystemTime, vec};

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

    let receiver = consumer.receiver();

    let test = r#"["458077348","457538965","436984755","495011834","492209327","475725102","435002485","459405194","458557053","511777214","510997488","433270583","434582858","467183757","467434844","511156923","530992169","503365046","515685543","510613065","536010433","463719680","454229150","483653392","458177948","430374411","517607152","429618127","452375274","475131894","465241807","442726969","475351326","443822464","429512698","453792951","433423037","490336353","447003083","445296280","525570026","453502232","440282032","455718349","485098147","458915030","499654421","471534330","465142253","428668124","461404421","490768260","476761540","505856063","430236692","481985442","474788566","461652655","513817440","479190846","459528682","450090994","467870337","429252743","467381107","475230594","428775939","537377628","507952946","488199078","430278611","429106416","450170012","447578131","433608743","430858533","463470223","467537453","431777439","448685016","498885294","432172175","447282345","453218038","474325084","443028707","429072053","432337462","431260875","490639746","494059187","515695223","537442670","435536701","511685609","431731727","428743207","473987705","438012431","458934115","509441591","520844019","536631495","430766328","435787384","452933414","429437023","444461737","507139302","452785223","536716713","444131780","455996543","477091225","464232589","428402665","465920344","502657543","477846179","517044692","506044478","429611478","495534468","429855022","510185237","525224533","485044372","503702838","429561553","455700335","433751710","536440902","440637413","446963320","476692176","488478319","451864183","448294241"]"#;

    loop {
        let message = receiver.recv().unwrap();
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                process_message(body.to_string()).await;
                //println!("(Received [{}]", body);
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

    let ids: Vec<u64> = deserialized
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!(
        "Getting ids starting at {} going to {}",
        &ids[0],
        &ids[ids.len() - 1]
    );

    let mut reqs = vec![];

    for i in &ids {
        let req = get_collections(*i);
        reqs.push(req);
    }

    println!("Making {} requests", reqs.len() * 2);

    // Proccess in 100 request chunks

    let c = trpl::join_all(reqs).await;

    push_data(c).await;

    let end = SystemTime::now();

    println!("Finished in {:?}", end.duration_since(start).unwrap());
}
