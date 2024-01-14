use std::sync::Arc;
use tokio::sync::mpsc;
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
    let (in_stream_tx, mut in_stream_rx) = mpsc::channel(128);

    tokio::spawn(async move {
        for i in 0..10 {
            in_stream_tx.send(i).await.unwrap();
        }
    });

    let mut set = JoinSet::new();

    let semaphore = Arc::new(Semaphore::new(3));

    loop {
        tokio::select! {
            n = in_stream_rx.recv() => {
                if n.is_none() {
                    break;
                }
                let semaphore = semaphore.clone();
                set.spawn(do_one(n.unwrap(), semaphore));
            },
        }
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(v) => match v {
                Ok(_) => println!("task succeeded"),
                Err(_) => {
                    println!("task failed");
                    set.abort_all()
                }
            },
            Err(_) => println!("task failed"),
        }
    }
}
