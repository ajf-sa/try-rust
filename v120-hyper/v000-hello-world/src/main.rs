use std::{net::SocketAddr, convert::Infallible};

use hyper::{service::{make_service_fn, service_fn}, Response, Body, Request, Server, header::HeaderValue, Method};

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
   let path = r.uri();
   match (r.method(),r.uri().path()) {
       (&Method::GET,"/") => {
         handle(r).await
       }
       _ =>  Ok(Response::new(Body::from("404")))
   }
  
}

async fn handle(_r: Request<Body>) -> Result<Response<Body>, Infallible> {
   let mut res = Response::default();
   res.headers_mut().insert(hyper::header::SET_COOKIE, HeaderValue::from_str("id=owifjoi").unwrap());
   *res.body_mut() = Body::from("owijefio");
   Ok(res)
}

