mod common;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute health check request");

    // assert
    assert!(response.status().is_success(), "Endpoint validity");
    assert_eq!(response.content_length(), Some(0));
}
