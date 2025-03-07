use std::io::{self, Write};
use std::{
    net::{IpAddr, TcpStream},
    sync::mpsc::{channel, Sender},
    thread,
};

use clap::Parser;

const MAX: u16 = 65535; // max number of ports

#[derive(Parser, Debug)]
struct Args {
    /// Ip address
    ip_addr: IpAddr,
}

fn scan(ip_addr: IpAddr, start_port: u16, num_threads: u16, tx: Sender<u16>) {
    let mut port: u16 = start_port + 1; // starting from port 1
    loop {
        if TcpStream::connect((ip_addr, port)).is_ok() {
            print!("."); // indicates a connection
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Args = Args::parse();
    let num_threads: u16 = 4;

    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx: Sender<u16> = tx.clone();

        thread::spawn(move || scan(args.ip_addr, i, num_threads, tx));
    }

    drop(tx); // drop transmitter from main thread

    let mut open_ports: Vec<u16> = Vec::new();

    for port in rx {
        open_ports.push(port);
    }

    open_ports.sort();

    println!("\n"); // to format output

    for port in open_ports {
        println!("PORT: {port} is open")
    }
}
