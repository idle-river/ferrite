use std::{
    fs,
    io::{self, Write},
};
use tokio::net::TcpStream;

use ferrite_core::{PORT_FILE, Packet, send};

#[tokio::main]
async fn main() {
    println!("Reading port from: {}", PORT_FILE);

    let port = fs::read_to_string(PORT_FILE)
        .unwrap_or("0".to_string())
        .parse::<u16>()
        .unwrap_or(0);

    let addr = format!("127.0.0.1:{}", port);

    let mut conn = TcpStream::connect(&addr).await.unwrap();

    println!("Connected to fer://{}", addr);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut raw_cmd = String::new();
        io::stdin().read_line(&mut raw_cmd).unwrap();

        let cmd: Vec<_> = raw_cmd.split_whitespace().collect();

        let operand = cmd[0];
        let args = &cmd[1..];

        let packet = match operand.to_lowercase().as_str() {
            "exit" => {
                break;
            }
            "help" => {
                println!("GET <key>\nSET <key> <value>\nDEL <key>\nEXISTS <key>");
                continue;
            }
            "get" => Packet::Get(args[0].to_string()),
            "set" => Packet::Set(args[0].to_string(), args[1].to_string()),
            _ => {
                eprintln!(
                    "{}: command not found\ntype HELP for a list of operands",
                    operand
                );
                continue;
            }
        };

        send(&mut conn, &packet).await.unwrap();
    }

    println!("Exiting...")
}
