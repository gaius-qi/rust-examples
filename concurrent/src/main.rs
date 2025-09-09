use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

async fn do_one(i: u32) -> Result<(), &'static str> {
    println!("------------ task {} received", i);
    if i == 5 {
        return Err("task failed");
    }

    println!("task {} started", i);
    if i % 2 == 0 {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    } else {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
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
                let permit = semaphore.acquire_owned().await.unwrap();
                set.spawn(async move {
                    let _permit = permit;
                    do_one(n.unwrap()).await
                });
            },
        }
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(v) => match v {
                Ok(_) => {}
                Err(_) => {
                    println!("task failed 1");
                    set.shutdown().await
                }
            },
            Err(err) => println!("task failed 2 {}", err),
        }
    }
}
