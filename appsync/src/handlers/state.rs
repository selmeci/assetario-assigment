use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::model::{AttributeValue, Select};
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::sync::Arc;

use crate::handlers::QueryBuilder;

pub struct StateQueryBuilder {
    pub client: Arc<Client>,
    pub state: String,
}

impl StateQueryBuilder {
    pub fn new(client: Arc<Client>, state: String) -> Self {
        println!("StateQueryBuilder::new");
        Self { client, state }
    }
}

impl QueryBuilder for StateQueryBuilder {
    fn build_query(&self, exclusive_start_key: Option<HashMap<String, AttributeValue>>) -> Query {
        self.client
            .query()
            .table_name("AssetarioSimpleMaps")
            .index_name("GSI1")
            .set_exclusive_start_key(exclusive_start_key)
            .key_condition_expression("#state = :state")
            .expression_attribute_names("#state", "G1PK")
            .expression_attribute_values(
                ":state",
                AttributeValue::S(format!("state#{}", self.state)),
            )
            .select(Select::AllAttributes)
    }
}
