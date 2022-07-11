use maplit::hashmap;
use rusoto_core::Region;
use rusoto_dynamodb::{BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, WriteRequest};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::structures::City;

const TABLE_NAME: &str = "AssetarioSimpleMaps";

pub struct DynamoDBWorker;

///
/// Worker save data in parallel thread
///
impl DynamoDBWorker {
    ///
    /// Send batch to DynamoDB until some date are provided.
    ///
    pub fn run(resp_tx: Sender<City>, retry_tx: Sender<Vec<City>>) -> Sender<Vec<City>> {
        let client = DynamoDbClient::new(Region::EuCentral1);
        let (tx, mut rx) = mpsc::channel::<Vec<City>>(100);
        let worker_tx = tx.clone();
        tokio::spawn(async move {
            println!("Worker started");
            while let Some(chunk) = rx.recv().await {
                let input = BatchWriteItemInput {
                    request_items: hashmap! {
                        TABLE_NAME.to_string() => chunk.iter().map(|city| {
                            WriteRequest {
                                put_request: Some(PutRequest {
                                    item: city.clone().into()
                                }),
                                ..WriteRequest::default()
                            }
                        }).collect()
                    },
                    ..BatchWriteItemInput::default()
                };
                if let Err(err) = client.batch_write_item(input).await {
                    println!("Cannot load data into DynamoDB{:?}", err);
                    if let Err(err) = retry_tx.send(chunk).await {
                        println!("Worker cannot send data into dead queue. err: {:?}", err);
                    }
                } else {
                    for city in chunk {
                        if let Err(err) = resp_tx.send(city).await {
                            println!("Worker cannot send response. {:?}", err);
                        };
                    }
                }
            }
            println!("Worker stopped");
        });
        worker_tx
    }
}
