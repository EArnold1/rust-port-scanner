use std::net::IpAddr;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Ip address
    pub ip_addr: IpAddr,

    /// Start of the range.
    #[arg(short = 's', long, default_value_t = 0)]
    pub start_port: u16,

    /// End of the range of ports to scan (inclusive).
    #[arg(short = 'e', long, default_value_t = u16::MAX)]
    pub end_port: u16,

    /// Number of threads to use.
    #[arg(short = 't', long, default_value_t = 4)]
    pub threads: usize,
}
