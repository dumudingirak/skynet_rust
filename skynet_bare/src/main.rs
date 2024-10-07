/**
 * This file contains the "bare" implementation of the skynet benchmark, using the tokio-framework only for issuing the asynchronous call in the main function.
 * Awaiting asnychronus tasks here is based on futures, without using the spawning and communication facilities of tokio directly.
 * Such implementation is inteded to be used mainly as an illustrative example of how the skynet-benchmark works in a general sense.
 */


use std::vec::Vec;
use futures::future::join_all;
/**
 * A simple struct to capsule the results of the benchmarks
 */
use benchmark::BenchmarkResults;
pub mod benchmark;

 async fn skynet(number:u64, size:u64, divisor:u64) -> u64{
    if size == 1{
        return number;
    }
    let mut sum:u64 = 0;
    let next_size:u64 = size/divisor;
    for i in 0..divisor{
        let child_number:u64 = number + i*next_size;
        let boxed = Box::pin(skynet(child_number, next_size, divisor)).await;
        sum += boxed;
    }
    return sum;
}

async fn vector_skynet(number:u64, size:u64, divisor:u64) -> u64 {
    if size == 1{
        return number;
    }
    let next_level:u64 = size/divisor;
    let to_await:Vec<_> = (0..divisor).map(|i| 
        vector_skynet(number + i*next_level, next_level, divisor)
    ).collect();
    let results = join_all(to_await).await;
    results.iter().sum()
}

async fn benchmark(times:u64) -> BenchmarkResults{
    let mut results:Vec<u128> = Vec::with_capacity(times.try_into().unwrap());
    for _ in 0..times{
        let start:std::time::Instant = std::time::Instant::now();
        let result = vector_skynet(0, 1_000_000, 10).await;
        let duration:std::time::Duration = start.elapsed();
        if result == 499_999_500_000{
            results.push(duration.as_nanos());
        }
    }
    return BenchmarkResults::new(results);
}

#[tokio::main]
async fn main(){
    let num_iterations = 2;
    let final_result = benchmark(num_iterations).await;
    final_result.print(num_iterations.try_into().unwrap());
}