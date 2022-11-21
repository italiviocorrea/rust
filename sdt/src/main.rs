use std::net::TcpListener;
use dotenv::dotenv;
use mongodb::Client;
use mongodb::options::ClientOptions;
use sdt::{run, Settings};


#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    // env_logger::init();
    //
    // let address = dotenv::var("address").unwrap();
    // let db_url = dotenv::var("db_url").unwrap();
    // let db_name = dotenv::var("db_name").unwrap();
    dotenv().ok();

    let cfg = Settings::new().unwrap();

    let listener = TcpListener::bind(&cfg.server.address).expect("Falha ao ligar ao listener");
    let mut client_options = ClientOptions::parse(cfg.database.url).await.expect("falha ao conectar ao servidor");
    client_options.app_name = Some("API_SDT".to_string());

    let client = Client::with_options(client_options).expect(" ");
    let db = client.database(&cfg.database.name.clone());

    println!("Server esta executando em {}",cfg.server.address);

    run(listener,db)?.await

}
