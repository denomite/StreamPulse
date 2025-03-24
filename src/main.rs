use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn run_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                let price = rand::random_range(100.0..200.0);
                let message = format!("Stock Price: {:.2}", price);

                if let Err(e) = socket.write_all(message.as_bytes()).await {
                    println!("Failed to write to socket: {}", e);
                    return;
                }

                match socket.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(n) => println!("Received: {}", String::from_utf8_lossy(&buffer[..n])),
                    Err(e) => {
                        println!("Failed to read from socket: {}", e);
                        return;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    run_server().await
}

/*
    Automated unit testing with tokio
    Simple test to verify the server responds
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
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

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
