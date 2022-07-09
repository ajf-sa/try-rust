use tokio::fs::File;
use tokio::io::BufReader;
use tokio::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub struct Response {
    pub writer: BufWriter<TcpStream>,
}

pub fn status(code: i32) -> &'static str {
    match code {
        200 => "OK",
        307 => "TEMPORARY REDIRECT",
        400 => "BAD REQUEST",
        404 => "NOT FOUND",
        _ => "NOT IMPLEMENTED",
    }
}

impl Response {
    pub fn new(client: TcpStream) -> Self {
        Response {
            writer: BufWriter::new(client),
        }
    }
    pub async fn write_status(&mut self, code: i32, status: &str) -> Result<usize> {
        self.writer
            .write(format!("HTTP/1.1 {} {}\r\n", code, status).as_bytes())
            .await
    }
    pub async fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer
            .write(format!("{}: {}\r\n", key, val).as_bytes())
            .await
    }

    pub async fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("Content-Length", val.len().to_string().as_str())
            .await?;
        self.writer.write(b"\n").await?;
        self.writer.write(val).await
    }
    pub fn mine_type(&self, key: &str) -> &str {
        if let Some((_, ext)) = key.rsplit_once(".") {
            match ext {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "text/javascript",
                "png" => "image/png",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "ico" => "image/x-icon",
                "pdf" => "application/pdf",
                _ => "text/plain",
            }
        } else {
            "text/plain"
        }
    }

    pub async fn write_file(&mut self, mut path: &str) -> Result<usize> {
        self.write_header("Connection", "Keep-Alive").await?;
        self.write_header("Keep-Alive", "timeout=5, max=100")
            .await?;
        self.write_header(
            "Content-Type",
            format!("{}; charset=utf-8", self.mine_type(path)).as_str(),
        )
        .await?;
        if path.starts_with("/") {
            path = &path[1..];
        }
        let file = match File::open(path).await {
            Ok(file) => file,
            Err(e) => {
                println!("{}:{}", path, e);
                return Ok(0);
            }
        };
        let mut buf = Vec::new();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buf).await?;

        self.write_body(&buf).await
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.writer.flush().await
    }
    pub async fn sendfile(&mut self, path: &str) -> Result<()> {
        self.write_file(path).await?;
        self.flush().await
       
    }
   
}
