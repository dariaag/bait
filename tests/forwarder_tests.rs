use bait::forwarder::forward_webhook;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_forward_webhook() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/test-endpoint"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let test_webhook_data = "{\"test\": \"data\"}".to_string();
    let test_url = format!("{}/test-endpoint", mock_server.uri());

    let result = forward_webhook(test_webhook_data, &test_url).await;
    assert!(result.is_ok(), "The webhook forwarding failed");
}
