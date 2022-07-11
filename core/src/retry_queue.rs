use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::structures::City;

pub struct RetryQueue;

///
/// Retry 3 times to load data if error occurs
///
impl RetryQueue {
    pub fn run(workers_tx: Sender<Vec<City>>) -> Sender<Vec<City>> {
        println!("Starting dead queue");
        let (tx, mut rx) = mpsc::channel::<Vec<City>>(100);
        let queue_tx = tx.clone();
        tokio::spawn(async move {
            let mut retries = HashMap::<u64, u8>::new();
            while let Some(chunk) = rx.recv().await {
                let mut hasher = DefaultHasher::default();
                chunk.hash(&mut hasher);
                let k = hasher.finish();
                let attempts = if let Some(attempts) = retries.get(&k) {
                    attempts + 1
                } else {
                    1
                };
                retries.insert(k, attempts);
                if attempts <= 3 {
                    if let Err(err) = workers_tx.send(chunk).await {
                        println!("Cannot retry job. err: {:?}", err);
                    }
                } else {
                    println!("Failed to load cities!");
                }
            }
            println!("Dead queue stopped");
        });
        queue_tx
    }
}
