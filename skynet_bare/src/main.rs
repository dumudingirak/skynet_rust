/**
 * This file contains the "bare" implementation of the skynet benchmark, using the tokio-framework only for issuing the asynchronous call in the main function
 * Awaiting asnychronus tasks here is based on futures, without using the spawning and communication facilities of tokio directly 
 */

 async fn skynet(number:u64, size:u64, divisor:u64) -> u64{
    if size == 1{
        return number;
    }
    let mut sum:u64 = 0;
    let next_size:u64 = size/divisor;
    for i in 0..divisor{
        let next_number:u64 = number + i*next_size;
        let boxed = Box::pin(skynet(next_number, next_size, divisor)).await;
        sum += boxed;
    }
    return sum;
}

#[tokio::main]
async fn main(){
    let start:std::time::Instant = std::time::Instant::now();
    let final_result = skynet(0, 1000000, 10).await;
    let duration:std::time::Duration = start.elapsed();
    let duration_ns = duration.as_nanos();
    println!("The final sum is: {} after a duration of {} ns", final_result, duration_ns);
}