use std::time::Duration;
use tokio::time::sleep;

async fn my_async_operation() -> String {
    println!("Starting async operation...");
    sleep(Duration::from_secs(1)).await;
    println!("Async operation in progress...");
    sleep(Duration::from_secs(1)).await;
    println!("Async operation still in progress...");
    sleep(Duration::from_secs(1)).await;
    println!("Async operation completed");
    "Operation completed".to_string()
}

#[tokio::main]
async fn main() {
    let timeout_duration = Duration::from_secs(3);

    tokio::select! {
        res = my_async_operation() => {
            println!("Operation finished: {}", res);
        }
        _ = sleep(timeout_duration) => {
            println!("Operation timed out after {:?}!", timeout_duration);
        }
    }

    sleep(Duration::from_secs(10)).await;
}
