use ferrite_core::{
    FerriteKV, PORT_FILE, Packet, Request, Response, is_disconnect_error, read, send,
};
use std::fs;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

pub(crate) type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to start server.");
    let store = Arc::new(Mutex::new(FerriteKV::new()));
    let port = listener
        .local_addr()
        .expect("Failed to get local addr")
        .port();

    // Temp value
    store
        .lock()
        .unwrap()
        .set("TEST".to_string(), 1234.to_string())
        .unwrap();

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

        let mut store_pointer = store.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut store_pointer, stream, addr).await {
                println!("[!] Error with client {}: {}", addr, e);
            };
        });
    }
}

async fn handle_connection(
    store: &mut Arc<Mutex<FerriteKV>>,
    mut stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Error> {
    loop {
        let packet = match read(&mut stream).await {
            Ok(packet) => packet,
            Err(error) => {
                if is_disconnect_error(&*error) {
                    println!("[-] Client disconnected: {}", addr);
                    break;
                }

                return Err(error);
            }
        };

        match packet {
            Packet::Request(p) => match p {
                Request::Get(key) => {
                    println!("[*] Client ({}) requested {}", addr, &key);
                    let value = {
                        let store = store.lock().unwrap();
                        store.get(&key).clone()
                    };
                    let packet = Response::Get(value);
                    send(&mut stream, &Packet::Response(packet)).await?;
                }
                Request::Set(key, value) => {
                    println!("Setting {}:{} into DB", key, value);
                }
                Request::Del(key) => {
                    println!("Deleting {} from DB", key);
                }
                Request::Exists(key) => {
                    println!("Checking if {} exists in DB", key);
                }
            },
            Packet::Ping => send(&mut stream, &Packet::Pong).await?,
            Packet::Pong => {}
            Packet::Response(_) => {
                eprintln!("[!] Server Packet Sent From {addr}");
                return Err("Server Packet Sent".into());
            }
        }
    }

    Ok(())
}
