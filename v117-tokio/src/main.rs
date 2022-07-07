use response::{Response, status};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;
use tokio::io::BufWriter;

mod response;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
   let  l = TcpListener::bind("127.0.0.1:8082").await?;
   loop {
   let (mut s,_) = l.accept().await?;
   let mut buf = [0; 1024];

  match s.read(&mut buf).await {
      Ok(count) if count == 0 => { println!("client closed connection"); break; },
      Ok(count) => {
        let req = std::str::from_utf8(&buf[0..count]).unwrap().split("\r\n").collect::<Vec<&str>>();
        let parts = req[0].split(" ").collect::<Vec<&str>>();
        if parts.len() < 2 {
            let mut res = Response::new(s);
            res.sendfile(400,status(400), "static/__400.html").await;
           return Ok(());
        }
        let mut res = Response::new(s);
        res.sendfile(200, status(200), "static/index.html").await;
        // handle(s,).await;
      },
      Err(err) => { println!("error: {}", err); break; },

  };
 }
 Ok(())
}

// async fn handle(stream: TcpStream)  {
//     let mut b:BufWriter<TcpStream> = BufWriter::new(stream); 
//     b.write(format!("HTTP/1.1 {} {}\r\n", 200, "OK").as_bytes()).await;
//     b.write(format!("Content-Type: {}; charset=utf-8","text/html").as_bytes()).await;

//     let body = "
//     <html>
//     <head>
//     <title>homepage</title>
//     </head>
//     <body>
//     <h1>hi</h1>
//     </body>
//     </html>
//     ".trim();
    
//     b.write(format!("Content-Length:{} \r\n",body.len().to_string().as_str()).as_bytes()).await;
//     b.write(format!("Connection:Keep-Alive\r\n").as_bytes()).await;
//     b.write(format!("Keep-Alive:timeout=5, max=100 \r\n").as_bytes()).await; 
//     b.write(format!("\n").as_bytes()).await;
//     b.write(format!("{}",body).as_bytes()).await;
//     b.flush().await;
    
// }

