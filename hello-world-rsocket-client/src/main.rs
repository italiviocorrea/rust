use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = RSocketFactory::connect()
        .acceptor(Box::new(|| Box::new(EchoRSocket)))
        .transport(TcpClientTransport::from("127.0.0.1:7878"))
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();
    let req = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();
    let res = cli.request_response(req).await.unwrap();
    println!("Obteve: {:?}", res);
    cli.close();
    Ok(())
}
