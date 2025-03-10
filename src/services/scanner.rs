use std::io::{self, Write};
use std::{
    net::{IpAddr, TcpStream},
    sync::mpsc::Sender,
};

#[derive(Clone)]
pub struct Scanner {
    ip_addr: IpAddr,
    end_port: u16, // max no. of ports
    pub threads: u16,
    // start_port: u16,
}

impl Scanner {
    pub fn new(ip_addr: IpAddr, threads: u16, end_port: u16) -> Scanner {
        Scanner {
            ip_addr,
            end_port,
            // start_port,
            threads,
        }
    }

    pub fn scan(&self, start_port: u16, tx: Sender<u16>) {
        let mut port: u16 = start_port + 1;
        loop {
            if TcpStream::connect((self.ip_addr, port)).is_ok() {
                print!("."); // indicates a connection
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }

            if (self.end_port - port) <= self.threads {
                break;
            }
            port += self.threads;
        }
    }
}
