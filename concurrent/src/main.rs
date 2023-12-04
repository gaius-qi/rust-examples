use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

async fn do_one(i: u32, semaphore: Arc<Semaphore>) -> Result<(), &'static str> {
    if i == 5 {
        return Err("task failed");
    }

    let _permit = semaphore.acquire().await.unwrap();
    println!("task {} started", i);
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    println!("task {} completed", i);
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    let semaphore = Arc::new(Semaphore::new(3));

    for i in 0..10 {
        let semaphore = semaphore.clone();
        set.spawn(do_one(i, semaphore));
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(v) => match v {
                Ok(_) => println!("task succeeded 1"),
                Err(_) => {
                    println!("task failed 1");
                    set.abort_all()
                }
            },
            Err(_) => println!("task failed 2"),
        }
    }
}
