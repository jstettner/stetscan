mod types;

use std::env;
use std::net::TcpListener;

mod tcp_interface;
use tcp_interface::{TcpInterface, Server, Client};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let host = &args[1];
    let port = &args[2];
    let mode: Box<dyn TcpInterface> = match args[3].as_str() {
        "s" | "server" => Box::new(Server),
        "c" | "client" => Box::new(Client),
        _ => panic!("Bad mode!"),
    };

    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    println!("TcpListener bound to {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => mode.handle_stream(&mut stream),
            Err(e) => println!("Failed to connect: {:?}", e),
        }
    }

    Ok(())
}
