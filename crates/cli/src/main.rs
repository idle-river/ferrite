use std::{
    fs,
    io::{self, Write},
    net::TcpStream,
};

use ferrite_core::PORT_FILE;

fn main() {
    println!("Reading port from: {}", PORT_FILE);

    let port = fs::read_to_string(PORT_FILE)
        .unwrap_or("0".to_string())
        .parse::<u16>()
        .unwrap_or(0);

    let addr = format!("127.0.0.1:{}", port);

    let mut conn = TcpStream::connect(&addr).unwrap();

    println!("Connected to fer://{}", addr);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut raw_cmd = String::new();
        io::stdin().read_line(&mut raw_cmd).unwrap();

        let cmd: Vec<_> = raw_cmd.split_whitespace().collect();

        let operand = cmd[0];
        let _args = &cmd[0..];

        match operand.to_lowercase().as_str() {
            "exit" => {
                break;
            }
            _ => conn.write_all(cmd.join(" ").as_bytes()).unwrap(),
        }
    }

    conn.shutdown(std::net::Shutdown::Both).unwrap();
    println!("Exiting...")
}
