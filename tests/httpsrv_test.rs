use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, TcpListener},
};

use zero2prod::{adapters::httpsrv, config::Config};

#[tokio::test]
async fn test_health_check() {
    // arrange
    let app_addr = spawn_app();

    // get a HTTP client
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", app_addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_subscribe_success() {
    // Arrange environment
    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let mut map = HashMap::new();
    map.insert("name", "Alice");
    map.insert("email", "alice@0xlab.xyz");
    let response = client
        .post(&format!("http://{}/subscriptions", &app_addr))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_subscribe_errors() {
    // Arrange environment
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        (r#"{"name": "Alice"}"#, "missing email"),
        (r#"{"email":"alice@0xlab.xyz"}"#, "missing name"),
        (r#"{}"#, "missing both name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app_addr))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

// Launch application in the background
// Returns local address of the HTTP server
fn spawn_app() -> String {
    // arrange config with port 0, means that unix-like system should assign a random port
    let cfg = Config {
        host: std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 0,
        log_level: "info".to_string(),
    };
    let listener = TcpListener::bind(SocketAddr::new(cfg.host, cfg.port))
        .expect("Failed to bind to random port");

    let local_addr = listener.local_addr().unwrap().to_string();

    let server = httpsrv::run(listener).expect("Failed to bind address");
    // run the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);

    local_addr
}
