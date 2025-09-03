use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use clap::Parser;

/// Simple program to scan port
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target ip address
    #[arg(short, long, default_value = "127.0.0.1"))]
    ip: String,
}

fn check_addr(ip: &str, port: u16) -> bool {
    let addr = format!("{ip}:{port}");
    if let Ok(mut addrs) = addr.to_socket_addrs(){
        if let Some(addr ) = addrs.next(){
            return TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok();
        }
    }
    false
}

fn main(){
    let args = Args::parse();

    let pb = indicatif::ProgressBar::new(65535);
    for port in 1..65535 {
        if check_addr(&args.ip, port) {
            pb.println(format!("[+] Open port: {}", port));
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");


}
