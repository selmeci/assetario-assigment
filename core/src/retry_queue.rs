use crate::load::LoadCommand;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::structures::City;

#[derive(Debug)]
pub enum RetryCommand {
    Retry(Vec<City>),
    End,
}

pub struct RetryQueue;

///
/// Retry 3 times to load data if error occurs
///
impl RetryQueue {
    pub fn run(workers_tx: Sender<LoadCommand>) -> Sender<RetryCommand> {
        println!("Starting retry queue");
        let (tx, mut rx) = mpsc::channel::<RetryCommand>(100);
        let queue_tx = tx.clone();
        tokio::spawn(async move {
            let mut retries = HashMap::<u64, u8>::new();
            while let Some(command) = rx.recv().await {
                match command {
                    RetryCommand::Retry(chunk) => {
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
                            if let Err(err) = workers_tx.send(LoadCommand::Load(chunk)).await {
                                println!("Cannot retry job. err: {:?}", err);
                            }
                        } else {
                            println!("Failed to load cities!");
                        }
                    }
                    RetryCommand::End => break,
                }
            }
            println!("Retry queue stopped");
        });
        queue_tx
    }
}
