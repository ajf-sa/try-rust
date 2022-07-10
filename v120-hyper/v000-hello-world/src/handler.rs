use std::convert::Infallible;

use hyper::{Request, Body, Response, header::HeaderValue};

pub async fn index(_r: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut res = Response::default();
    res.headers_mut().insert(hyper::header::SET_COOKIE, HeaderValue::from_str("id=owifjoi").unwrap());
    *res.body_mut() = Body::from("owijefio");
    Ok(res)
 }

 pub async fn blog(r: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut res = Response::default();
    res.headers_mut().insert(hyper::header::SET_COOKIE, HeaderValue::from_str("id=owifjoi").unwrap());
    *res.body_mut() = Body::from("owijefio");
    Ok(res)
 }