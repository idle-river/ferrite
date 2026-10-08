use ferrite_core::{FerriteKV, PORT_FILE, Packet, read};
use std::fs;
use tokio::net::{TcpListener, TcpStream};

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
    let packet = read(&mut stream).await?;

    match packet {
        Packet::Get(key) => {
            println!("Getting {} from DB", key);
        }
        Packet::Set(key, value) => {
            println!("Setting {}:{} into DB", key, value);
        }
        Packet::Del(key) => {
            println!("Deleting {} from DB", key);
        }
        Packet::Exists(key) => {
            println!("Checking if {} exists in DB", key);
        }
    }

    Ok(())
}
