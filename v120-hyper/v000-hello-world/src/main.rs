use std::{net::SocketAddr, convert::Infallible, io};

use hyper::{service::{make_service_fn, service_fn}, Response, Body, Request, Server,  Method};

mod handler;
use handler::{index, blog};
use routerify::{Router, ext::RequestExt, Middleware, RouterService};
#[tokio::main(flavor = "current_thread")]
async fn main() {
   let router = router();
   let service = RouterService::new(router).unwrap();
   let addr = SocketAddr::from(([0,0,0,0],8082));
   let server = Server::bind(&addr).serve(service);
   if let Err(e) = server.await {
      eprintln!("server error: {}", e);
  }
}



async fn logger_middleware(req: Request<Body>) -> Result<Request<Body>, io::Error> {
   println!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
   Ok(req)
}

fn router() -> Router<Body, io::Error> {
   Router::builder().middleware(Middleware::pre(logger_middleware))
   .get("/", index)
   .get("/blog", blog)
   .get("/blog/:id", blog)
   .build()
   .unwrap()
}

