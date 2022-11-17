mod common;

#[tokio::test]
async fn greet_works() {
    // arrange
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/hello", &address))
        .send()
        .await
        .expect("Failed to execute hello world request");

    // assert
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
