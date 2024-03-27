use lazy_static::lazy_static;
use std::{collections::VecDeque, time::Duration};
use tokio::{sync::Mutex, time::interval};

use crate::forwarder;

pub struct WebhookItem {
    payload: String,
    url: String,
}

lazy_static! {
    pub static ref QUEUE: Mutex<VecDeque<WebhookItem>> = Mutex::new(VecDeque::new());
}

pub async fn enqueue_webhook(payload: String, url: String) {
    let webhook_item = WebhookItem { payload, url };
    let mut queue = QUEUE.lock().await;
    queue.push_back(webhook_item);
}

pub async fn dequeue_webhook() -> Option<WebhookItem> {
    let mut queue = QUEUE.lock().await;
    queue.pop_front()
}

pub async fn start_processing_loop() {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if let Some(webhook_item) = dequeue_webhook().await {
            if let Err(e) =
                forwarder::forward_webhook(webhook_item.payload, &webhook_item.url).await
            {
                eprintln!("Error forwarding webhook to {}: {}", webhook_item.url, e);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime; // Import Runtime to support async tests

    #[test]
    fn enqueue_dequeue_webhook() {
        // Create a new Tokio runtime for testing
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            // Ensure the queue is empty before testing
            while dequeue_webhook().await.is_some() {}

            // Test data
            let payload = "Test payload".to_string();
            let url = "http://example.com/webhook".to_string();

            // Enqueue the webhook item
            enqueue_webhook(payload.clone(), url.clone()).await;

            // Dequeue and verify the item
            if let Some(webhook_item) = dequeue_webhook().await {
                assert_eq!(webhook_item.payload, payload, "Payload does not match");
                assert_eq!(webhook_item.url, url, "URL does not match");
            } else {
                panic!("Dequeued item was None");
            }
        });
    }
}
