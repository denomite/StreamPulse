# StreamPulse

RealTime Server is a high-performance, real-time stock price streaming server built in Rust.
It fetches live data from Finnhub’s API and broadcasts it to connected clients with minimal latency, showcasing Rust’s concurrency and efficiency. Designed as a fintech foundation, it powers applications an average person might use—like investment trackers, price alerts, or educational tools—delivering real-time financial insights in a scalable, reliable way.

## Features

<ins>Real-Time Stock Streaming:</ins> Updates prices (e.g., AAPL at $219.68) every 5 seconds using Finnhub’s API.  
<ins>High Concurrency:</ins> Handles thousands of clients via Tokio’s async runtime and broadcast channels.  
<ins>Low Latency:</ins> Delivers data with ~5s fetch intervals, extensible to sub-second with WebSocket.  
<ins>Scalable Design:</ins> One API call serves all clients, optimized for free-tier limits (60 calls/min).

## Tech stack

**Rust**: Core language for speed, safety, and zero-cost abstractions.  
**Tokio**: Async runtime for concurrent I/O and task management.  
**Reqwest**: HTTP client for fetching Finnhub data.  
**Serde**: JSON parsing for stock quotes.  
**Finnhub API**: Real-time stock price source.

## Project development

1. **Foundation & Server Design**  
   <ins>Objective: Build a TCP server for real-time data streaming.</ins>

    Implementation:  
     Initialized with cargo new realtime_server --bin.  
     Structured as a multi-binary project: src/bin/main.rs (server) and src/bin/client.rs (benchmark client).  
     Used tokio::net::TcpListener to accept connections.

2. **Concurrent Programming**

    <ins>Objective: Scale to multiple clients efficiently.</ins>

    Implementation:
    Employed tokio::spawn and Semaphore (1000 connections) for parallel client handling.
    Added tokio::sync::broadcast to share one API fetch with all clients.

3. **Fintech Integration with Finnhub**

    <ins>Objective: Stream real stock prices for fintech relevance.</ins>

    Implementation:  
    Swapped simulated data (rand) for Finnhub’s /quote endpoint (e.g., AAPL: $219.68).  
    Fetches every 5s (12/min), staying under free-tier 60/min limit.  
    Added Arc<Mutex> to cache the latest price for instant client delivery.

4. **Performance Optimization**

    <ins>Low Latency:</ins>

    BufWriter batches writes, reducing I/O overhead.  
    5s fetch interval, tunable to 1s (60/min max) or sub-second with WebSocket.  
    Memory Efficiency: Pre-allocated buffers and reused strings.  
    Benchmarking: client.rs measures throughput (18.63 bytes/sec) and latency (1449ms/message).

5. **Testing**

    <ins>Objective: Ensure reliability and accuracy.</ins>

    Implementation:  
    client.rs simulates 3 clients, each reading 10 updates (e.g., 810 bytes in 43.48s).  
    Validates Finnhub data parsing and broadcast delivery.

### Real-World Application

<ins>Real-World Applications</ins>  
For the average person, this server could power:  
Investment Trackers: Live portfolio updates on your phone (e.g., "AAPL: $219.68").  
Price Alerts: Notifications when stocks hit targets (e.g., "Buy AAPL at $215").  
Budgeting Tools: Real-time net worth with stock holdings.

### Performance Metrics

    Benchmark (3 Clients):
    Processed: 810 bytes
    Duration: 43.48s
    Throughput: 18.63 bytes/sec
    Latency: 1449.32ms/message (fetch-limited)
    Scalability: Supports 1000+ clients with one API call.
