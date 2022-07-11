mod handlers;

use crate::handlers::country::CountryQueryBuilder;
use crate::handlers::state::StateQueryBuilder;
use crate::handlers::{Handler, QueryBuilders};
use aws_sdk_dynamodb::Client;
use core::cache::Cache;
use core::structures::GqlTree;
use futures_util::TryFutureExt;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

fn event_arg<'a>(event: &'a LambdaEvent<Request>, name: &str) -> Option<&'a String> {
    event.payload.arguments.get(name)
}

fn new_query_builder(
    client: Arc<Client>,
    field_name: &str,
    state: &str,
    country: &str,
) -> Arc<QueryBuilders> {
    Arc::new(if field_name.eq("state") {
        QueryBuilders::State(StateQueryBuilder::new(
            Arc::clone(&client),
            state.to_string(),
        ))
    } else {
        QueryBuilders::Country(CountryQueryBuilder::new(
            Arc::clone(&client),
            state.to_string(),
            country.to_string(),
        ))
    })
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(rename = "fieldName")]
    field_name: String,
}

#[derive(Debug, Deserialize)]
struct Request {
    arguments: HashMap<String, String>,
    info: Info,
}

///
/// If response is cached in S3, return cache, else calculate tree from DynamoDB data
///
async fn function_handler(event: LambdaEvent<Request>) -> Result<GqlTree, Error> {
    println!("{:#?}", event);
    let shared_config = aws_config::load_from_env().await;
    let client = Arc::new(Client::new(&shared_config));
    let field_name = event.payload.info.field_name.as_str();
    let resp = match field_name {
        "tree" => Cache::get("tree").await?.unwrap_or(vec![]),
        "state" | "country" => {
            let all_countries = String::from("all");
            let state = event_arg(&event, "state_name").expect("Missing required arg");
            let country = event_arg(&event, "country_name").unwrap_or(&all_countries);
            let cache_key = format!("{}-{}", state, country);
            match Cache::get(&cache_key).await? {
                None => {
                    println!("Missing in cache");
                    let handler = Handler {
                        query_builder: new_query_builder(client, field_name, state, country),
                    };
                    handler
                        .handle()
                        .and_then(|tree| async move {
                            Cache::store(cache_key, &tree).await?;
                            Ok(tree)
                        })
                        .await?
                }
                Some(tree) => {
                    println!("Returning from cache");
                    tree
                }
            }
        }
        _ => {
            println!("Unknown query");
            vec![]
        }
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
