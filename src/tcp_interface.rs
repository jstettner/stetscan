use crate::types::Byte;

use std::net::TcpStream;

pub trait TcpInterface {
    fn handle_stream(&self, stream: &mut TcpStream);
}

pub struct Server;
pub struct Client;

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

