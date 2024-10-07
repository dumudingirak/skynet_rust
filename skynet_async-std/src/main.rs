/**
 * And yet one more variant of the skynet benchmark, here using the async_std framework
 * Change the value of num_iterations in order to perform muliple benchmarks in a row.
 */

use async_recursion::async_recursion;
use benchmark::BenchmarkResults;
use futures::future::join_all;
use async_std::task;
pub mod benchmark;

#[async_recursion]
async fn vector_skynet(number:u64, size:u64, divisor:u64) -> u64 {
    if size == 1{
        return number;
    }
    let next_level:u64 = size/divisor;
    let to_await:Vec<_> = (0..divisor).map(|i| 
        task::spawn(vector_skynet(number + i*next_level, next_level, divisor))
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
        if result == 499_999_500_000 {
            results.push(duration.as_nanos());
        }
    }
    return BenchmarkResults::new(results);
}

#[async_std::main]
async fn main(){
    let num_iterations = 2;
    let final_result = benchmark(num_iterations).await;
    final_result.print(num_iterations.try_into().unwrap());
}