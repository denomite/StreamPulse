/*
    Simple custom benchmark(Windows Friendly)
    Run client in another terminal:
    - cargo run --release --bin client
*/
use std::time::Instant;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Starting client...");
    let start = Instant::now();
    let mut handles = Vec::new();

    for i in 0..3 {
        handles.push(tokio::spawn(async move {
            println!("Client {} connecting...", i);
            match TcpStream::connect("127.0.0.1:8080").await {
                Ok(mut stream) => {
                    println!("Client {} connected", i);
                    let mut buffer = [0; 1024];
                    let mut total_bytes = 0;

                    for _ in 0..10 {
                        match stream.read(&mut buffer).await {
                            Ok(n) => {
                                total_bytes += n;
                                println!(
                                    "Client {} read: {}",
                                    i,
                                    String::from_utf8_lossy(&buffer[..n])
                                );
                            }
                            Err(e) => {
                                println!("Client {} read error: {}", i, e);
                                return total_bytes;
                            }
                        }
                    }
                    println!("Client {} finished", i);
                    total_bytes
                }
                Err(e) => {
                    println!("Client {} failed to connect: {}", i, e);
                    0
                }
            }
        }));
    }

    println!("Waiting for all clients to finish...");
    let mut total_bytes = 0;
    let all_tasks = futures::future::join_all(
        handles
            .into_iter()
            .map(|handle| tokio::time::timeout(tokio::time::Duration::from_secs(5), handle)),
    )
    .await;

    for (i, result) in all_tasks.into_iter().enumerate() {
        match result {
            Ok(Ok(bytes)) => total_bytes += bytes,
            Ok(Err(e)) => println!("Client {} task failed: {}", i, e),
            Err(_) => println!("Client {} timed out", i),
        }
    }

    let duration = start.elapsed().as_secs_f64();
    println!("Processed {} bytes in {:.2}s", total_bytes, duration);
    println!("Throughput: {:.2} bytes/sec", total_bytes as f64 / duration);
    println!(
        "Avg latency per message: {:.2}ms",
        (duration * 1000.0) / (3.0 * 10.0)
    );
    Ok(())
}
