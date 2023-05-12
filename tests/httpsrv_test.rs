use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, TcpListener},
};

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    adapters::httpsrv,
    config::{Config, DbConfig},
};

#[tokio::test]
async fn test_health_check() {
    // arrange
    let app = spawn_app().await;

    // get a HTTP client
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_subscribe_success() {
    // Arrange environment
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let mut map = HashMap::new();
    map.insert("name", "Alice");
    map.insert("email", "alice@0xlab.xyz");
    let response = client
        .post(&format!("http://{}/subscriptions", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_conn_pool)
        .await
        .expect("Failed to fetch saved subcription");

    assert_eq!(saved.email, "alice@0xlab.xyz");
    assert_eq!(saved.name, "Alice");
}

#[tokio::test]
async fn test_subscribe_errors() {
    // Arrange environment
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        (r#"{"name": "Alice"}"#, "missing email"),
        (r#"{"email":"alice@0xlab.xyz"}"#, "missing name"),
        (r#"{}"#, "missing both name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app.address))
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

pub struct TestApp {
    pub address: String,
    pub db_conn_pool: PgPool,
}

// Launch application in the background
// Returns local address of the HTTP server and database connection string
async fn spawn_app() -> TestApp {
    // arrange config with port 0, means that unix-like system should assign a random port
    let mut cfg = Config {
        host: std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 0,
        log_level: "info".to_string(),
        db: DbConfig {
            username: "postgres".to_string(),
            password: "password".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            name: "newsletter".to_string(),
        },
    };

    // randomize database name for a test
    cfg.db.name = Uuid::new_v4().to_string();

    let listener = TcpListener::bind(SocketAddr::new(cfg.host, cfg.port))
        .expect("Failed to bind to random port");

    let local_addr = listener.local_addr().unwrap().to_string();

    let db_conn_pool = configure_db(&cfg.db).await;

    let server = httpsrv::run(listener, db_conn_pool.clone()).expect("Failed to bind address");
    // run the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);

    TestApp {
        address: local_addr,
        db_conn_pool,
    }
}

async fn configure_db(cfg: &DbConfig) -> PgPool {
    let mut db_conn = PgConnection::connect(&cfg.server_connection_string())
        .await
        .expect("Failed to connect database server");

    db_conn
        .execute(format!(r#"CREATE DATABASE "{}";"#, cfg.name).as_str())
        .await
        .expect("Failed to create database");

    // migrate database
    let db_conn_pool = PgPool::connect(&cfg.connection_string())
        .await
        .expect("Failed to connected to database");

    sqlx::migrate!("./migrations")
        .run(&db_conn_pool)
        .await
        .expect("Failed database migration");

    db_conn_pool
}
