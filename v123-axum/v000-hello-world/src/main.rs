
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(|| async { Html("<h1>Hello, World!</h1>")})).route("/blog", get(|| async {
        Html(std::include_str!("./views/index.html"))
    }));
  
    // run it
   serve(app).await;
 
}
async fn serve(app: Router,) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}