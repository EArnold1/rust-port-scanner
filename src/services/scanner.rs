use std::io::{self, Write};
use std::{
    net::{IpAddr, TcpStream},
    sync::mpsc::Sender,
};

#[derive(Clone)]
pub struct Scanner {
    ip_addr: IpAddr,
    pub end_port: u16, // max no. of ports
    pub threads: usize,
    pub start_port: u16,
}

impl Scanner {
    pub fn new(ip_addr: IpAddr, threads: usize, end_port: u16, start_port: u16) -> Scanner {
        Scanner {
            ip_addr,
            end_port,
            start_port,
            threads,
        }
    }
    pub fn scan(&self, port: u16, tx: Sender<u16>) {
        if TcpStream::connect((self.ip_addr, port)).is_ok() {
            print!("."); // indicates a connection
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
    }
}
