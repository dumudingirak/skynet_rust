# skynet_rust
This repository contains two implementations for the skynet concurrency-benchmark in Rust, using two frameworks for the execution of asynchronous tasks, namely tokio and async-std. 
Skynet was originally invented by Alexander Temerev, whose original implementations can be found at https://github.com/atemerev/skynet


Our approach mainly differs from the original implementation in that it only does not use coroutine-rs, but other, well-known and stable frameworks
