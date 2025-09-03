use std::net::ToSocketAddrs;
use std::time::Duration;
use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::sync::Arc;


/// Simple program to port scan
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target ip address
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,
    /// Connection timeout in ms
    #[arg(short, long, default_value = "250")]
    to: u64,
    /// thread count
    #[arg(short, long, default_value = "100")]
    ct: usize,
}

// ip:port connection checker
async fn check_addr(ip: &str, port: u16, timeout_ms: u64) -> Option<u16> {

    let addr = format!("{ip}:{port}");

    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await {
                Ok(Ok(_)) => return Some(port),
                _ => return None
            }
        }
    }

    None
}

#[tokio::main]
async fn main(){
    // parse args
    let args = Args::parse();

    // init progress bar
    let pb = indicatif::ProgressBar::new(65534);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/white}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );

    // init semaphore
    let semaphore = Arc::new(tokio::sync::Semaphore::new(args.ct));

    // init tasks vector
    let mut tasks = Vec::new();

    // set each task ready to run
    for port in 1..=65534 {
        let ip = args.ip.clone();
        let timeout_ms = args.to;
        let permit = semaphore.clone();

        // spawn task
        let task = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            check_addr(&ip, port, timeout_ms).await
        });
        tasks.push(task);
    }

    // run tasks
    for task in tasks {
        if let Ok(Some(port)) = task.await {
            pb.println(format!("{}:{}", args.ip, port));
        }
        pb.inc(1);
    }
    pb.finish_with_message("Scan completed!");
}