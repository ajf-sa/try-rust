use std::convert::Infallible;

use hyper::{Request, Body, Response, header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE}, StatusCode};

pub async fn index(_r: Request<Body>) -> Result<Response<Body>, Infallible> {
   Ok(make_response(&"index", StatusCode::OK))
 }

 pub async fn blog(r: Request<Body>) -> Result<Response<Body>, Infallible> {
   Ok(make_response(&"blog", StatusCode::OK)) 
 }

 fn make_response(body: &'static str, status: StatusCode) -> Response<Body> {
   Response::builder()
       .status(status)
       .header(CONTENT_LENGTH, body.len() as u64)
       .header(CONTENT_TYPE, "text/plain")
       .body(Body::from(body))
       .expect("Failed to construct response")
}