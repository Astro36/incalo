use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use time::OffsetDateTime;

async fn process(mut stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let now = OffsetDateTime::now_utc();

    let msg = "Hello, World!";
    let res = format!(
        "HTTP/1.1 200 Ok\r\n\
        Date: {}\r\n\
        Server: incalo\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        now.format("%a, %d %b %Y %H:%M:%S GMT"),
        msg.len(),
        msg
    );

    stream.write_all(res.as_bytes()).await?;

    Ok(())
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", listener.local_addr()?);

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async {
            process(stream).await.unwrap();
        });
    }

    Ok(())
}
