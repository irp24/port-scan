use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

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
    let ip = env::args().nth(1).unwrap_or_else(|| "127.0.0.1".into());
    let ports = [];

    for port in ports {
        if check_addr(&ip, port) {
            println!("Open port: {port}")
        }
    }
}