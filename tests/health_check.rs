use zero2prod::adapters::httpsrv;

#[tokio::test]
async fn test_health_check() {
    // arrange
    spawn_app();

    // get a HTTP client
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch application in the background
fn spawn_app() {
    let server = httpsrv::run().expect("Failed to bind address");
    // run the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);
}
