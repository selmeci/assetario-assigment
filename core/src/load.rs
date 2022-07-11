use anyhow::Result;
use futures_core::Stream;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;

use crate::retry_queue::RetryQueue;
use crate::structures::City;
use crate::worker::DynamoDBWorker;

const P_TASKS: usize = 4;

#[derive(Default)]
pub struct Loader;

impl Loader {
    fn start_workers(
        mut rx: Receiver<Vec<City>>,
        resp_tx: Sender<City>,
        dead_queue: Sender<Vec<City>>,
    ) {
        println!("Starting workers");
        tokio::spawn(async move {
            // initialize workers
            let mut workers = (0..P_TASKS)
                .map(|_| DynamoDBWorker::run(resp_tx.clone(), dead_queue.clone()))
                .collect::<Vec<_>>();
            let mut chunk = 0;
            while let Some(cities) = rx.recv().await {
                let worker = &workers[chunk % P_TASKS];
                chunk += 1;
                println!("Loading cities...");
                //wait here if all workers are busy
                if let Err(err) = worker.send(cities.clone()).await {
                    println!("Cannot send chunk to workers. err: {:?}", err);
                    // replace failed worker with new one
                    let worker = DynamoDBWorker::run(resp_tx.clone(), dead_queue.clone());
                    workers[(chunk - 1) % P_TASKS] = worker;
                    // send chunk to dead queue
                    if let Err(err) = dead_queue.send(cities).await {
                        println!("Cannot send chunk into dead queue. err: {:?}", err);
                    }
                }
            }
        });
    }

    ///
    /// Load data into DynamoDB with 4 parallel tasks.
    ///
    /// Return channel reader which contains loaded cities into DynamoDB
    ///
    pub async fn load(
        data: impl Stream<Item = City> + Send + 'static,
    ) -> Result<impl Stream<Item = City>> {
        println!("Loading data");
        let (resp_tx, rx) = mpsc::channel::<City>(1000);
        let (workers_tx, workers_rx) = mpsc::channel::<Vec<City>>(1000);
        let retry_queue = RetryQueue::run(workers_tx.clone());
        Self::start_workers(workers_rx, resp_tx, retry_queue.clone());
        // load data in separate thread
        tokio::spawn(async move {
            // prepare data chunks
            let jobs = data
                // DynamoDB allow 25 items in batch operation and we want 4 parallel writes
                .chunks(25)
                .chunks(P_TASKS);
            let job_stream = jobs.into_inner();
            tokio::pin!(job_stream);
            while let Some(cities) = job_stream.next().await {
                if let Err(err) = workers_tx.send(cities.clone()).await {
                    println!("Cannot send chunk to workers. err: {:?}", err);
                    if let Err(err) = retry_queue.send(cities).await {
                        println!("Cannot send chunk to dead queue. err {:?}", err);
                    }
                }
            }
            println!("CSV stream end");
        });
        Ok(ReceiverStream::new(rx))
    }
}
