use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, Semaphore, broadcast};

/*
    Run client in  terminal:
    - cargo run --release --bin main
*/

// Structs added to parse JSON response
#[derive(Debug, Serialize, Deserialize)]
struct FinnhubQuote {
    #[serde(rename = "c")]
    current_price: f64,
    #[serde(rename = "h")]
    high_price: f64,
    #[serde(rename = "l")]
    low_price: f64,
    #[serde(rename = "o")]
    open_price: f64,
    #[serde(rename = "pc")]
    prev_close: f64,
    #[serde(rename = "t")]
    timestamp: i64,
}

async fn fetch_stock_price(symbol: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, api_key
    );
    let response = reqwest::get(&url).await?;
    let raw_text = response.text().await?;
    println!("Raw API response: {}", raw_text);

    match serde_json::from_str::<FinnhubQuote>(&raw_text) {
        Ok(quote) => Ok(format!(
            "Stock price ({}): {}\n",
            symbol, quote.current_price
        )),
        Err(e) => Ok(format!("Stock price ({}): Parse Error - {}\n", symbol, e)),
    }
}

async fn run_server() -> tokio::io::Result<()> {
    dotenv().ok();
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
    let api_key = std::env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY must be set");
    let symbol = "AAPL";

    let (tx, _) = broadcast::channel(16);
    let tx = Arc::new(tx);
    let last_price = Arc::new(Mutex::new("Stock Price (AAPL): Initializing\n".to_string()));

    // Single fetcher task
    {
        let tx = tx.clone();
        let last_price = last_price.clone();
        tokio::spawn(async move {
            loop {
                match fetch_stock_price(symbol, &api_key).await {
                    Ok(price) => {
                        let mut last = last_price.lock().await;
                        *last = price.clone();
                        let _ = tx.send(price);
                    }
                    Err(e) => {
                        let error_msg = format!("Error fetching price: {}\n", e);
                        let mut last = last_price.lock().await;
                        *last = error_msg.clone();
                        let _ = tx.send(error_msg);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }

    // Handle client connections
    loop {
        let _permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        let tx = tx.clone();
        let last_price = last_price.clone();
        tokio::spawn(async move {
            /*
            Buffer Writes (tokio::io::BufferWriter): This reduces system calls, lowering latency.
             */
            let mut writer = BufWriter::new(socket);
            let mut rx = tx.subscribe();

            // Send last known price immediately, with proper error handling
            let initial_result: Result<(), std::io::Error> = async {
                let last = last_price.lock().await;
                println!("Sending initial to {}: {}", addr, last.trim());
                writer.write_all(last.as_bytes()).await?;
                writer.flush().await?;
                Ok(())
            }
            .await;

            if let Err(e) = initial_result {
                println!("Failed to send initial to {}: {}", addr, e);
                return; // Exit task if initial send fails
            }

            // Broadcast loop
            let result: Result<(), std::io::Error> = async {
                loop {
                    match rx.recv().await {
                        Ok(message) => {
                            println!("Sending to {}: {}", addr, message.trim());
                            writer.write_all(message.as_bytes()).await?;
                            writer.flush().await?;
                        }
                        Err(e) => {
                            println!("Receive error for {}: {}", addr, e);
                            break;
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Ok(())
            }
            .await;

            if let Err(e) = result {
                println!("Client {} disconnected: {}", addr, e);
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
