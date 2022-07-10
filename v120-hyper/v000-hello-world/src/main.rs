use std::{net::SocketAddr, convert::Infallible};

use hyper::{service::{make_service_fn, service_fn}, Response, Body, Request, Server,  Method};

mod handler;
use handler::{index, blog};
#[tokio::main(flavor = "current_thread")]
async fn main() {
   let addr = SocketAddr::from(([0,0,0,0],8082));
   let make_svc = make_service_fn(|_conn| async {
      Ok::<_,Infallible>(service_fn(route))
   });
   let server = Server::bind(&addr).serve(make_svc);
   if let Err(e) = server.await {
      eprintln!("server error: {}", e);
  }
}

async fn route (r : Request<Body>) ->  Result<Response<Body>, Infallible>  {
   match (r.method(),r.uri().path()) {
       (&Method::GET,"/") => {
         index(r).await
       }
       (&Method::GET,"/blog") =>{
         blog(r).await
       }
       _ =>  Ok(Response::new(Body::from("404")))
   }
}



