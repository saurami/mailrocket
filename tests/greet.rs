#[tokio::test]
async fn greeting_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/hello")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(
        Some(12),
        response.content_length(),
        "Length of 'Hello, World!' is 12"
    );
}

fn spawn_app() {
    let server = mailrocket::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
