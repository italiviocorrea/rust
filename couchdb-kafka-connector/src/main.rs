use std::env;
use std::error::Error;
use std::string::String;
use std::time::Duration;

use futures_util::{StreamExt};
use log::info;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use serde_json::{Value};
use tokio::time;
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Le o ultimo sequencial caso exista.
    let mut last_seq: Option<Value> = get_last_seq().await;
    println!("last_seq = {:?}", last_seq);

    // conecta no banco de dados
    let client = couch_rs::Client::new(&*couchdb_host(), &*couchdb_username(), &*couchdb_password())?;
    let db = client.db(&*couchdb_database()).await?;

    let mut changes = db.changes(Option::from(last_seq.clone()));

    changes.set_infinite(true);

    let producer_delay_error: i32 = kafka_producer_delay_error_ms().parse().unwrap();

    while let Some(change) = changes.next().await {

        let docs = change.unwrap();
        last_seq = Option::from(Value::from(docs.seq));
        println!("{:?} last_seq = {:?}", docs.doc, last_seq);

        let message = docs.doc.unwrap().to_string();

        // Esse loop assegura que a mensagem será enviada para o kafka
        loop {
            let produce_future = produce(message.clone());
            match produce_future.await {
                Ok(delivery) => {
                    println!("Mensagem enviada com sucesso para o kafka {:?}", delivery);
                    // Grava o ultimo sequencial no zookeeper
                    save_last_seq(last_seq.unwrap()).await;
                    break;
                }
                Err((e, _)) => {
                    println!("Kafka Erro {:?}", e);
                    if producer_delay_error > 0 {
                        println!("Atraso de {} ms, para nova tentativa de conexão com o kafka cluster ", producer_delay_error);
                        time::sleep(time::Duration::from_millis(producer_delay_error as u64)).await;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn produce(message: String) -> OwnedDeliveryResult {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &*kafka_brokers_urls())
        .set("message.timeout.ms", kafka_message_timeout_ms())
        .create()
        .expect("Producer creation error");
    let topic_nome = &*kafka_topic();
    let produce_future = producer
        .send(
            FutureRecord::to(topic_nome)
                .key("some key")
                .payload(&message),
            Duration::from_secs(0),
        );

    produce_future.await
}



async fn get_last_seq() -> Option<Value> {
    let zk_urls = zk_server_urls();
    println!("connecting to {}", zk_urls);

    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();
    zk.add_listener(|zk_state| println!("New ZkState is {:?}", zk_state));
    return match zk.get_data(&*zk_path(), true) {
        Ok(x) => Option::from(Value::from(String::from_utf8(x.0).unwrap().replace("\"", ""))),
        Err(_) => Option::from(None),
    };
}

async fn save_last_seq(last_seq: Value) {
    let zk_urls = zk_server_urls();
    println!("connecting to {}", zk_urls);

    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();
    zk.add_listener(|zk_state| println!("New ZkState is {:?}", zk_state));
    let exists = zk.exists(&*zk_path(), true).unwrap();
    match exists {
        None => {
            println!("{:?}", Vec::from(last_seq.to_string()));
            let _path = zk.create(&*zk_path(),
                                  Vec::from(last_seq.to_string()),
                                  Acl::open_unsafe().clone(),
                                  CreateMode::Persistent);
        }
        Some(_) => { let _set_data = zk.set_data(&*zk_path(), Vec::from(last_seq.to_string()), None); }
    }
}

fn couchdb_host() -> String {
    let key = "COUCHDB_HOST";
    env::var(key).unwrap_or_else(|_| "http://localhost:5984".to_string())
}

fn couchdb_database() -> String {
    let key = "COUCHDB_DATABASE";
    env::var(key).unwrap_or_else(|_| "test_db".to_string())
}

fn couchdb_username() -> String {
    let key = "COUCHDB_USERNAME";
    env::var(key).unwrap_or_else(|_| "admin".to_string())
}

fn couchdb_password() -> String {
    let key = "COUCHDB_PASSWORD";
    env::var(key).unwrap_or_else(|_| "senha#123".to_string())
}

fn zk_server_urls() -> String {
    let key = "ZOOKEEPER_SERVERS";
    env::var(key).unwrap_or_else(|_| "localhost:2181".to_string())
}

fn zk_path() -> String {
    let key = "ZOOKEEPER_PATH";
    env::var(key).unwrap_or_else(|_| "/bcad_last_seq".to_string())
}

fn kafka_brokers_urls() -> String {
    let key = "KAFKA_BROKERS";
    env::var(key).unwrap_or_else(|_| "localhost:9092,localhost:9093,localhost:9094".to_string())
}

fn kafka_topic() -> String {
    let key = "KAFKA_TOPIC_NAME";
    env::var(key).unwrap_or_else(|_| "kafka_test".to_string())
}

fn kafka_producer_delay_error_ms() -> String {
    let key = "KAFKA_PRODUCER_DELAY_ERROR_MS";
    env::var(key).unwrap_or_else(|_| 5000.to_string())
}

fn kafka_message_timeout_ms() -> String {
    let key = "KAFKA_MESSAGE_TIMEOUT_MS";
    env::var(key).unwrap_or_else(|_| 5000.to_string())
}
