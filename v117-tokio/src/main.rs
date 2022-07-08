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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let l = TcpListener::bind("0.0.0.0:8082").await?;
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
                    res.sendfile(400, status(400), "static/__400.html").await;
                    return Ok(());
                }
                
                match (parts[0], parts[1]) {
                    ("GET",  path) => {
                        let mut  path = path.split("?").collect::<Vec<&str>>()[0];
                        if path.len()  >  1{
            
                        if path.chars().last().unwrap() == '/' {
                            let mut chars = path.chars();
                            chars.next_back();
                            path = chars.as_str();
                        }
                    }
                        let mut res = Response::new(s);
                        let r = Regex::new("^/$").unwrap();
                        if r.is_match(path) {
                            res.write_status(307, status(307)).await?;
                            res.write_header("Location", "/blog").await?;
                            res.flush().await;
                            // res.write_header("Set-Cookie","id=woief;HttpOnly; SameSite=Lax; Secure").await?; 
                            // res.write_file("static/index.html").await?;
                            // res.flush().await?;
                        }
                        let r = Regex::new("^/blog$").unwrap();
                        if r.is_match(path) {
                            res.sendfile(200, status(200), "static/blog/index.html")
                                .await;
                        }
                        let r = Regex::new("^/blog/([0-9a-zA-Z]{1,30})$").unwrap();
                        if r.is_match(path) {
                            println!("{}", path[6..].to_string());
                            res.sendfile(200, status(200), "static/blog/index.html")
                                .await;
                        }

                        res.sendfile(200, status(200), "static/__404.html").await;
                    }
                    _ => {
                        let mut res = Response::new(s);
                        res.sendfile(200, status(200), "static/__404.html").await;
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
