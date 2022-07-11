use crate::{CountryQueryBuilder, GqlTree, StateQueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_stream::stream;
use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::model::AttributeValue;
use core::transform::Transformer;
use futures_util::TryFutureExt;
use serde_dynamo::from_items;

pub mod country;
pub mod state;

pub trait QueryBuilder {
    fn build_query(&self, exclusive_start_key: Option<HashMap<String, AttributeValue>>) -> Query;
}

pub enum QueryBuilders {
    State(StateQueryBuilder),
    Country(CountryQueryBuilder),
}

impl QueryBuilder for QueryBuilders {
    fn build_query(&self, exclusive_start_key: Option<HashMap<String, AttributeValue>>) -> Query {
        match self {
            QueryBuilders::State(builder) => builder.build_query(exclusive_start_key),
            QueryBuilders::Country(builder) => builder.build_query(exclusive_start_key),
        }
    }
}

pub struct Handler<B: QueryBuilder + Send + Sync + 'static> {
    pub query_builder: Arc<B>,
}

impl<B: QueryBuilder + Send + Sync + 'static> Handler<B> {
    pub async fn handle(&self) -> Result<GqlTree> {
        let query_builder = Arc::clone(&self.query_builder);
        let cities = stream! {
            let mut exclusive_start_key = None;
            loop {
                if let Ok(resp) = query_builder
                    .build_query(exclusive_start_key.clone())
                    .send()
                    .await
                {
                    if let Some(items) = resp.items {
                        println!("resp.{}", items.len());
                        if let Ok(cities) = from_items(items) {
                            for city in cities {
                                yield city;
                            }
                        };
                    }
                    if let Some(last_evaluated_key) = resp.last_evaluated_key {
                        exclusive_start_key = Some(last_evaluated_key)
                    } else {
                        break;
                    }
                } else {
                    break;
                };
            }
        };
        Transformer::transform(cities)
            .and_then(Transformer::gql_tree)
            .await
    }
}
