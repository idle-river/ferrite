use colored::*;
use ferrite_cli::{Command, command};
use ferrite_core::{PORT_FILE, Packet, send};
use std::{
    fs,
    io::{self, Write},
};
use tokio::net::TcpStream;

pub(crate) const COMMANDS: [Command; 4] = [
    command!("GET", ["<key>"], "Gets a key from the DB"),
    command!("SET", ["<key>", "<value>"], "Sets a key in the DB"),
    command!("DEL", ["<key>"], "Deletes a key from the DB"),
    command!("HELP", "Prints this message"),
];

#[tokio::main]
async fn main() {
    println!("{}", format!("Reading port from: {}", PORT_FILE).dimmed());

    let port = fs::read_to_string(PORT_FILE)
        .unwrap_or("0".to_string())
        .parse::<u16>()
        .unwrap_or(0);

    let addr = format!("127.0.0.1:{}", port);

    let mut conn = TcpStream::connect(&addr).await.unwrap();

    println!("{}", format!("Connected to fer://{}", addr).dimmed().cyan());

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut raw_cmd = String::new();
        io::stdin().read_line(&mut raw_cmd).unwrap();

        let cmd: Vec<_> = raw_cmd.split_whitespace().collect();

        if cmd.len() < 2 {
            println!("Failed to parse: less than 2 args");
            continue;
        }

        let operand = cmd[0];
        let args = &cmd[1..];

        let packet = match operand.to_lowercase().as_str() {
            "exit" => {
                break;
            }
            "help" => {
                print_help();
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

fn print_help() {
    let max_len = COMMANDS
        .iter()
        .map(|command| {
            command.usage.len()
                + command
                    .parameters
                    .iter()
                    .map(|parameter| parameter.len() + 1)
                    .sum::<usize>()
        })
        .max()
        .unwrap_or(0);

    for command in COMMANDS {
        let parameters = command.parameters.join(" ");

        let usage_len = command.usage.len()
            + if parameters.is_empty() {
                0
            } else {
                parameters.len() + 1
            };

        print!("{}", command.usage.green());

        if !parameters.is_empty() {
            print!(" {}", parameters.cyan());
        }

        print!("{:width$}", "", width = max_len - usage_len + 2);

        println!("{}", command.description.yellow());
    }
}
