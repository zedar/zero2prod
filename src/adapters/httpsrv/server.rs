use std::net::TcpListener;

use actix_web::{
    dev::Server, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

// Responds with Hello {name}! or Hello World! if no /{name} is providec.
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello: {}!", name)
}

// Responds OK (HTTP 200)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize, Debug)]
struct Subscription {
    name: String,
    email: String,
}

// Subscribes a new user
// If some fields are missing the 400 Bad Request is returned automatically
// Before calling `subscribe` `actix_web` invokes the `from_request` for the input parameter. The
// `from_request` tries to deserialize the body into a JSON
async fn subscribe(
    subscription: web::Json<Subscription>,
    db_conn_pool: web::Data<PgPool>,
) -> HttpResponse {
    let req_id = Uuid::new_v4();

    log::info!("request: {}, new subscription: {:?}", req_id, subscription);

    let res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1,$2,$3,$4)
        "#,
        req_id,
        subscription.email,
        subscription.name,
        Utc::now()
    )
    // get_ref() i used to take an immutable reference
    // sqlx has an async interface, but it does not allow to run concurrent queries
    .execute(db_conn_pool.get_ref())
    .await;

    match res {
        Ok(_) => {
            log::info!(
                "request {}, subscription added: {}, {}",
                req_id,
                subscription.email,
                subscription.name
            );
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log::error!(
                "request {}, failed to register subscription: {}, {}, error: {}",
                req_id,
                subscription.email,
                subscription.name,
                err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Creates an HTTP server that should be called with an await keyword
pub fn run(listener: TcpListener, db_conn_pool: PgPool) -> anyhow::Result<Server> {
    // wrap the database connection pool in a smart pointer e.k.a Arc::
    let db_conn_pool = web::Data::new(db_conn_pool);
    let local_addr = listener.local_addr()?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            // get a smart pointer and attach it to the application state
            .app_data(db_conn_pool.clone())
    })
    .listen(listener)?
    .run();

    log::info!("[HTTP_SERVER] running: {:?}", local_addr);

    Ok(server)
}
