#[tokio::test]
async fn greeting_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/hello")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success(), "Endpoint validity");
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/plain; charset=utf-8"
    );
    assert_eq!(
        response.content_length(),
        Some(13),
        "Response length is 13 characters"
    );
    assert_eq!(
        response.text().await.unwrap(),
        "Hello, World!",
        "Response from endpoint"
    );
}

fn spawn_app() {
    let server = mailrocket::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
