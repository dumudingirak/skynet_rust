/**
 * The skynet-benchmark, invented by Alexander Temerev (https://github.com/atemerev/skynet), implemented with the Rust-based tokio framework for asynchronus execution.
 * Increase num_iterations, in order to increase the accuracy of the results.
 */
use tokio::sync::mpsc;
use async_recursion::async_recursion;
use benchmark::BenchmarkResults;
pub mod benchmark;

/**
 * The actual benchmark function: If the size of partial tree, starting with this root invokation is greater than 1, spawn divisor child tasks, which are called with size = size/divisor and 
 * Use channels for communicating between child tasks and the father task
 */
#[async_recursion]
async fn skynet(tx:tokio::sync::mpsc::Sender<u64>, number:u64, size:u64, divisor:u64){
    if size == 1{
        let _ = tx.send(number).await;
        return
    }
    let next_size:u64 = size/divisor;
    let (own_tx, mut rx) = mpsc::channel(10);
    for i in 0..divisor{
        let next_number:u64 = number + i*next_size;
        let new_tx = own_tx.clone();
        tokio::spawn(skynet(new_tx, next_number, next_size, divisor));
    }
    //Important: If the tasks own send handle is kept, the channel will never be closed
    drop(own_tx);
    //Finally, sum over all resulting values of the child tasks
    let mut sum:u64 = 0;
    while let Some(result) = rx.recv().await  {
        sum += result;
    }
    let _ = tx.send(sum).await;
}

async fn benchmark(times:u64) -> BenchmarkResults{
    let mut results:Vec<u128> = Vec::with_capacity(times.try_into().unwrap());
    for _ in 0..times{
        let mut result:u64 = 0;
        //Initialize the benchmarks return channel
        let (bench_tx, mut bench_rx) = mpsc::channel(1);
        //Start the timer
        let start:std::time::Instant = std::time::Instant::now();
        //The actual benchmark: Spawn a new skynet task with target size 1000000, divided by 10 each for each level
        tokio::spawn(async move{skynet(bench_tx, 0, 1_000_000, 10).await});
        while let Some(sum) = bench_rx.recv().await {
            result = sum;
        }
        let duration:std::time::Duration = start.elapsed();
        //If the result obtained by execution of the benchmark does not fit (i.e. the benchmark failed), discard it
        if result == 499_999_500_000 {
            results.push(duration.as_nanos());
        }
    }
    //Return the results
    return BenchmarkResults::new(results);
}

#[tokio::main]
async fn main(){
    let num_iterations = 2;
    let benchmark_result = benchmark(num_iterations).await;
    benchmark_result.print(num_iterations.try_into().unwrap());
}