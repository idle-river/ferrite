use ferrite_core::{FerriteKV, PORT_FILE};
use std::fs;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub(crate) type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to start server.");
    let _store = FerriteKV::new();
    let port = listener
        .local_addr()
        .expect("Failed to get local addr")
        .port();

    if let Err(error) = fs::write(PORT_FILE, port.to_string()) {
        eprintln!("Failed to write port number to {}: {error}", PORT_FILE);
    }

    println!("[*] Ferrite server listening on 127.0.0.1:{port}");

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        println!("[+] New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                println!("[!] Error with client {}: {}", addr, e);
            };
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0u8; 1024];
    #[allow(clippy::unused_io_amount)]
    stream.read(&mut buf).await?;

    loop {
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        stream.write_all(&buf[..n]).await?;

        println!("[*] New msg from client: {}", String::from_utf8_lossy(&buf));
    }

    Ok(())
}
