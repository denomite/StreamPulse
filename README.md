# StreamPulse

    StreamPulse is a high-performance, real-time data processing server built in Rust.
    Designed to handle thousands of concurrent connections with minimal latency, it processes
    and streams data(currenlty simulated stock prices) to clients efficiently. This project
    demonstrates advanced systems programming, concurent task management and performance
    optimization using Rust ecosystem.

## Features

    Real-time data streaming: Delivers data to clients with configurable latency
    (curently 5ms, optimizable to <1ms).
    High concurrency: Handles thousands of connections using Tokio's async runtime.
    Peformance optimized: Minimizes latency and memory usage.
    Tested reliability: Includes automated tests.
