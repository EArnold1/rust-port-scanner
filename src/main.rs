use std::{
    net::IpAddr,
    sync::mpsc::{channel, Sender},
    thread,
};

use clap::Parser;
use services::scanner::Scanner;

mod services;

#[derive(Parser, Debug)]
struct Args {
    /// Ip address
    ip_addr: IpAddr,
}

fn main() {
    let args: Args = Args::parse();

    let (tx, rx) = channel();

    let new_scanner: Scanner = Scanner::new(args.ip_addr);

    let threads = new_scanner.threads;

    for i in 0..threads {
        let new_scanner = new_scanner.clone();
        let tx: Sender<u16> = tx.clone();

        thread::spawn(move || {
            new_scanner.scan(i, tx);
        });
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
