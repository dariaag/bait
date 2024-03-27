// src/main.rs

mod forwarder;
mod queue;
mod receiver;
#[actix_web::main]
async fn main() {
    tokio::spawn(async {
        queue::start_processing_loop().await;
    });

    receiver::start_server()
        .await
        .expect("Failed to start the server");
}
