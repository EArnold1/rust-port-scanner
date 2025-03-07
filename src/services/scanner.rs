use std::io::{self, Write};
use std::{
    net::{IpAddr, TcpStream},
    sync::mpsc::Sender,
};

const MAX: u16 = 65535; // max number of ports

#[derive(Clone)]
pub struct Scanner {
    ip_addr: IpAddr,
    max: u16, // max no. of ports
    pub threads: u16,
}

impl Scanner {
    pub fn new(ip_addr: IpAddr) -> Scanner {
        Scanner {
            ip_addr,
            max: MAX,
            threads: 4,
        }
    }

    pub fn scan(&self, start_port: u16, tx: Sender<u16>) {
        let mut port: u16 = start_port + 1; // starting from port 1
        loop {
            if TcpStream::connect((self.ip_addr, port)).is_ok() {
                print!("."); // indicates a connection
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }

            if (self.max - port) <= self.threads {
                break;
            }
            port += self.threads;
        }
    }
}
