use std::{convert::Infallible, io, net::SocketAddr};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, body, header,
};

mod handler;
use handler::{blog, index, protect_page, login};
use routerify::{ext::RequestExt, Middleware, Router, RouterService, RequestInfo};
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    let server = Server::bind(&addr).serve(service);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn logger_middleware(req: Request<Body>) -> Result<Request<Body>, io::Error> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

async fn protect(mut res: Response<Body>, req_info: RequestInfo) -> Result<Response<Body>, io::Error>{
    let cookie = req_info
    .headers()
    .get(header::COOKIE)
    .and_then(|v| v.to_str().ok())
    .unwrap_or("");
   let id = cookie.split("=").collect::<Vec<&str>>();
   if id[1] == "admin" {
    return Ok(res);
   }
   Ok(Response::new(Body::from("not allow!")))
}

fn router() -> Router<Body, io::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .get("/", index)
        .get("/login",login)
        .get("/blog", blog)
        .get("/blog/:id", blog)
        .scope("/api", api::router())
        .build()
        .unwrap()
}


mod api {
    use super::*;



    pub fn router() -> Router<Body, io::Error> {
        // Create a router for API and specify the the handlers.
        Router::builder().middleware(Middleware::post_with_info(protect))
            .get("/admin", protect_page)
            .build()
            .unwrap()
    }
}