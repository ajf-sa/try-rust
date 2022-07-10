use hyper::{StatusCode, Response, Body, header::{CONTENT_LENGTH, CONTENT_TYPE}};

pub fn index() -> Response<Body> {
    println!("WEf");
    make_response(&"woeifj", StatusCode::OK)
}
fn make_response(body: &'static str, status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}