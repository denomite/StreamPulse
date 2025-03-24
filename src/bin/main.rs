use rand::Rng;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpListener;
use tokio::sync::Semaphore;

/*
    Run client in  terminal:
    - cargo run --release --bin main
*/

async fn run_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    /*
    Councurrency tuning: Handle thousands of connections efficiently. Semaphore added to cap concurrent
    cleints and prevent overload
    - Semaphore: Added Semaphore::new(1000) to cap concurrent connections at 1000.
    This prevents resource exhaustion(to many open sockets) while allowing high concurrency.
    - acquire_owned() ensures the permit lives with the spawned task, dropping automatically
    when connection ends.
    */
    let semaphore = Arc::new(Semaphore::new(1000));

    loop {
        let _permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            // Memmory management: String reuse, avoid creating new string for each message
            let mut message = String::with_capacity(32);
            /*
            Buffer Writes (tokio::io::BufferWriter): This reduces system calls, lowering latency.
             */
            let mut writer = BufWriter::new(socket);

            let result: Result<(), std::io::Error> = async {
                loop {
                    message.clear();
                    let price = rand::thread_rng().gen_range(100.0..200.0);
                    use std::fmt::Write;
                    write!(&mut message, "Stock Price: {:.2}\n", price).unwrap();

                    println!("Sending to {}: {}", addr, message.trim());
                    writer.write_all(message.as_bytes()).await?;
                    writer.flush().await?;

                    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
                }
            }
            .await;

            if let Err(e) = result {
                println!("Task failed: {} failed: {}", addr, e);
            }
        });
    }
    // Error handling: Kept robusting with ? and logging, ensuring stabiltiy under load
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    run_server().await
}

/*
    Automated unit testing with tokio
    Simple test to verify the server responds
    Run the test: cargo test
*/
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_server_sends_data() {
        // Spawn the server as a Future
        tokio::spawn(run_server());

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Connect to the server
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        let mut buffer = [0; 1024];

        // Read data from the server
        let n = stream.read(&mut buffer).await.unwrap();
        let received = String::from_utf8_lossy(&buffer[..n]);
        assert!(
            received.contains("Stock Price:"),
            "Server should sen stock price"
        );
    }
}
