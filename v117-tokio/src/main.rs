use regex::Regex;
use response::{status, Response};
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
mod response;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}
// comment

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let l = TcpListener::bind("127.0.0.1:8082").await?;
    loop {
        let (mut s, _) = l.accept().await?;
        let mut buf = [0; 1024];

        match s.read(&mut buf).await {
            Ok(count) if count == 0 => {
                println!("client closed connection");
                break;
            }
            Ok(count) => {
                let req = std::str::from_utf8(&buf[0..count])
                    .unwrap()
                    .split("\r\n")
                    .collect::<Vec<&str>>();
                let parts = req[0].split(" ").collect::<Vec<&str>>();
                if parts.len() < 2 {
                    let mut res = Response::new(s);
                    res.write_status(400, status(400)).await;
                    res.sendfile( "static/__400.html").await;
                    return Ok(());
                }

                match (parts[0], parts[1]) {
                    ("GET", path) => {
                        let mut path = path.split("?").collect::<Vec<&str>>()[0];
                        if path.len() > 1 {
                            if path.chars().last().unwrap() == '/' {
                                let mut chars = path.chars();
                                chars.next_back();
                                path = chars.as_str();
                            }
                        }
                        let mut res = Response::new(s);
                        let r = Regex::new("^/$").unwrap();
                        if r.is_match(path) {
                            res.write_status(200, status(200)).await;
                            res.write_header("Set-Cookie", "id=woiejf;").await;
                            res.sendfile("static/index.html").await;
                        }
                        let r = Regex::new("^/blog$").unwrap();
                        if r.is_match(path) {
                            res.write_status(200, status(200)).await;
                            res.sendfile("static/blog/index.html")
                                .await;
                        }
                        let r = Regex::new("^/blog/([0-9a-zA-Z]{1,30})$").unwrap();
                        if r.is_match(path) {
                            println!("{}", path[6..].to_string());
                            res.write_status(200, status(200)).await;
                            res.sendfile("static/blog/index.html")
                                .await;
                        }
                        res.write_status(404, status(404)).await;
                        res.sendfile("static/__404.html").await;
                    }
                    _ => {
                        let mut res = Response::new(s);
                        res.write_status(404, status(404)).await;
                        res.sendfile("static/__404.html").await;
                    }
                }
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        };
    }
    Ok(())
}
