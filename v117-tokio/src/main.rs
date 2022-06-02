use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        handle(socket).await;
    }
}

async fn handle(mut strem: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let mut buf = vec![0; 1024];

        loop {
            let n = match strem.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let cmd = std::str::from_utf8(&buf[0..n]).unwrap();
            println!("{}", cmd);

            strem.write_all(&buf[0..n]).await;
        }
    });
    Ok(())
}
