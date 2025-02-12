use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the server")
}

async fn catch_all() -> impl Responder {
    HttpResponse::Ok().body("HELLo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/{tail:.*}", web::get().to(catch_all))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
