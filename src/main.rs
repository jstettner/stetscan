use std::env;
use std::net::{TcpListener, TcpStream};

trait TcpInterface {
    fn handle_stream(&self, stream: &mut TcpStream);
}

struct Server;
struct Client;

type Byte = u8;

const PACKET_SIZE: usize = 256;

impl TcpInterface for Server {
    fn handle_stream(&self, stream: &mut TcpStream) {
        let mut data: Vec<Byte> = vec![];
        let mut buffer = [0u8; PACKET_SIZE];
        //loop {
        //}
    }
}

impl TcpInterface for Client{
    fn handle_stream(&self, stream: &mut TcpStream) {
        let mut data: Vec<Byte> = vec![];
        let mut buffer = [0u8; PACKET_SIZE];
        //loop {
        //}
    }
}

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
