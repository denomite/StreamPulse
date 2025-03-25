# StreamPulse

RealTime Server is a high-performance, real-time stock price streaming server built in Rust. It fetches live data from Finnhub’s API and broadcasts it to connected clients with minimal latency, showcasing Rust’s concurrency and efficiency. Designed as a fintech foundation, it powers applications an average person might use—like investment trackers, price alerts, or educational tools—delivering real-time financial insights in a scalable, reliable way.

## Features

Real-Time Stock Streaming: Updates prices (e.g., AAPL at $219.68) every 5 seconds using Finnhub’s API.
High Concurrency: Handles thousands of clients via Tokio’s async runtime and broadcast channels.
Low Latency: Delivers data with ~5s fetch intervals, extensible to sub-second with WebSocket.
Scalable Design: One API call serves all clients, optimized for free-tier limits (60 calls/min).

### Tech stack

**Rust**: Core language for speed, safety, and zero-cost abstractions.
**Tokio**: Async runtime for concurrent I/O and task management.
**Reqwest**: HTTP client for fetching Finnhub data.
**Serde**: JSON parsing for stock quotes.
**Finnhub API**: Real-time stock price source.
