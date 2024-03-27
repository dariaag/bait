use crate::queue;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(receive_webhook))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[post("/webhook")]
async fn receive_webhook(body: String) -> impl Responder {
    println!("Received webhook: {}", body);
    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let destination = json_body["destination"].as_str().unwrap().to_string();
    queue::enqueue_webhook(body, destination).await;
    HttpResponse::Ok().body("Webhook received")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::JsonBody::Body;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_rt::test]
    async fn test_receive_webhook() {
        let payload = serde_json::json!({
            "body": "Test webhook content",
            "destination": "http://example.com/webhook/endpoint"
        });

        let mut app = test::init_service(App::new().service(receive_webhook)).await;

        let req = test::TestRequest::post()
            .uri("/webhook")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // Optionally, verify if the queue receives the correct data if your mocking setup allows it
    }
}
