use hyper::body::HttpBody as _;
use hyper::{header, Client};
use hyper::{Body, Method, Request};
use tokio::io::{stdout, AsyncWriteExt as _};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = Client::new();
    let bo = "full_name=1&email=1&mobile=1&job_no=1&specialty=1&work_years=1&cats=1&current_dpet=1&old_exp=1&submit=1";
    let mut req = Request::builder()
        .method(Method::POST)
        .header(header::CONTENT_LENGTH, bo.len())
        .uri("x")
        .body(Body::from(bo))
        .expect("request builder");
    let mut resp = client.request(req).await?;

    println!("Response: {},{:?}", resp.status(), resp.body());

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    Ok(())
}
