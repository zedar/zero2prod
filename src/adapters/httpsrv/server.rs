use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// Responds with Hello {name}! or Hello World! if no /{name} is providec.
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello: {}!", name)
}

// Responds OK (HTTP 200)
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// Creates an HTTP server that should be called with an await keyword
pub fn run() -> anyhow::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
