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

    /// Start of the range.
    #[arg(short = 's', long, default_value_t = 1)]
    start_port: u16,

    /// End of the range of ports to scan (inclusive).
    #[arg(short = 'e', long, default_value_t = 65535)]
    end_port: u16,

    /// Number of threads to use.
    #[arg(short = 't', long, default_value_t = 4)]
    threads: u16,
}

fn main() {
    let args: Args = Args::parse();

    let (tx, rx) = channel();

    let new_scanner: Scanner = Scanner::new(args.ip_addr, args.threads, args.end_port);

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

    println!("\n"); // for output display

    for port in open_ports {
        println!("PORT: {port} is open")
    }
}
