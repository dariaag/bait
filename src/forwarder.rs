use reqwest::Error;

pub async fn forward_webhook(webhook: String, url: &str) -> Result<(), Error> {
    // Example URL - replace with your actual destination or use a dynamic approach

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(webhook)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Webhook forwarded successfully");
    } else {
        // Log or handle the error accordingly
        eprintln!("Failed to forward webhook. Status: {}", res.status());
    }

    Ok(())
}
