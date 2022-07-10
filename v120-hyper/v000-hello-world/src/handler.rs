use std::io;

use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE, self, HeaderValue},
    Body, Request, Response, StatusCode,
};
use routerify::ext::RequestExt;

pub async fn index(_r: Request<Body>) -> Result<Response<Body>, io::Error> {
    Ok(make_response(&"index", StatusCode::OK))
}

pub async fn protect_page(_r: Request<Body>) -> Result<Response<Body>, io::Error> {
    Ok(make_response(&"protect", StatusCode::OK))
}

pub async fn login(_r: Request<Body>) -> Result<Response<Body>, io::Error> {
   let mut res = Response::<Body>::default();
   res.headers_mut()
   .insert(header::SET_COOKIE, HeaderValue::from_str("id=admin;").unwrap());
    Ok(res)
}

pub async fn blog(r: Request<Body>) -> Result<Response<Body>, io::Error> {
    let id = match r.param("id") {
        Some(x) => x,
        None => "0",
    };
    println!("{id}");
    let body = format!(
        "
   <html>
   <head>
   <title>{}</title>
   </head>
   </html>
   ",
        id
    );
    Ok(make_response(
        Box::leak(body.into_boxed_str()),
        StatusCode::OK,
    ))
}

fn make_response(body: &'static str, status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(body))
        .expect("Failed to construct response")
}
