use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(index2))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn index2(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}
