use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}", &name)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/hello", web::get().to(greet)))
        .bind("127.0.0.1:8080")?
        .run();
    Ok(server)
}
