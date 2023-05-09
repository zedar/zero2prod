use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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
async fn subscribe(subscription: web::Json<Subscription>) -> HttpResponse {
    log::info!("{:?}", subscription);
    HttpResponse::Ok().finish()
}

// Creates an HTTP server that should be called with an await keyword
pub fn run(listener: TcpListener) -> anyhow::Result<Server> {
    let local_addr = listener.local_addr()?;
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    log::info!("[HTTP_SERVER] running: {:?}", local_addr);

    Ok(server)
}
