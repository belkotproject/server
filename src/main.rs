use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("188.245.245.17", 80))?
    .run()
    .await
}
