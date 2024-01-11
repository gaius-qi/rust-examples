use leaky_bucket::RateLimiter;
use std::time;

#[tokio::main]
async fn main() {
    let limiter = RateLimiter::builder()
        .initial(1024)
        .refill(1024)
        .interval(std::time::Duration::from_secs(1))
        .fair(false)
        .build();

    let start = time::Instant::now();

    println!("Waiting for permit...");

    // Should take ~400 ms to acquire in total.
    let a = limiter.acquire(1024);
    let b = limiter.acquire(1024);
    let c = limiter.acquire(1024);

    let ((), (), ()) = tokio::join!(a, b, c);

    println!(
        "I made it in {:?}!",
        time::Instant::now().duration_since(start)
    );
}
