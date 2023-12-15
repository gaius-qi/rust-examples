use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let example = Example::new();
    let example = Arc::new(example);

    let mut set = JoinSet::new();
    for _ in 0..10 {
        let example = example.clone();
        set.spawn(async move {
            example.do_something().await.unwrap();
        });
    }

    while let Some(res) = set.join_next().await {
        println!("Task completed with result {:?}", res);
    }

    Ok(())
}

struct Example {
    mutex: Mutex<()>,
}

impl Example {
    pub fn new() -> Self {
        Self {
            mutex: Mutex::new(()),
        }
    }

    pub async fn do_something(&self) -> Result<(), Box<dyn Error>> {
        let Ok(_guard) = self.mutex.try_lock() else {
            println!("Mutex is locked");
            return Ok(());
        };

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Do something");
        Ok(())
    }
}
