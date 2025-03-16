use std::sync::{
    mpsc::{channel, Sender},
    Arc,
};

use clap::Parser;
use services::{args::Args, scanner::Scanner, worker_pool::WorkerPool};

mod services;

fn main() {
    let args: Args = Args::parse();

    let Args {
        end_port,
        start_port,
        threads,
        ip_addr,
    } = args;

    let (tx, rx) = channel();

    let scanner: Scanner = Scanner::new(ip_addr, threads, end_port, start_port);

    let pool: WorkerPool = WorkerPool::new(scanner.threads);

    let scanner: Arc<Scanner> = Arc::new(scanner); // for shared ownership

    for port in scanner.start_port..=scanner.end_port {
        let tx: Sender<u16> = tx.clone();
        let scanner: Arc<Scanner> = Arc::clone(&scanner);

        pool.execute(move || {
            scanner.scan(port, tx);
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
