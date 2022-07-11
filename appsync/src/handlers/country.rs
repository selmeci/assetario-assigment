use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::model::{AttributeValue, Select};
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::sync::Arc;

use crate::handlers::QueryBuilder;

pub struct CountryQueryBuilder {
    pub client: Arc<Client>,
    pub state: String,
    pub country: String,
}

impl CountryQueryBuilder {
    pub fn new(client: Arc<Client>, state: String, country: String) -> Self {
        println!("CountryQueryBuilder::new");
        Self {
            client,
            state,
            country,
        }
    }
}

impl QueryBuilder for CountryQueryBuilder {
    fn build_query(&self, exclusive_start_key: Option<HashMap<String, AttributeValue>>) -> Query {
        self.client
            .query()
            .table_name("AssetarioSimpleMaps")
            .index_name("GSI1")
            .set_exclusive_start_key(exclusive_start_key)
            .key_condition_expression("#state = :state AND #country = :country")
            .expression_attribute_names("#state", "G1PK")
            .expression_attribute_names("#country", "G1SK")
            .expression_attribute_values(
                ":state",
                AttributeValue::S(format!("state#{}", self.state)),
            )
            .expression_attribute_values(
                ":country",
                AttributeValue::S(format!("country#{}", self.country)),
            )
            .select(Select::AllAttributes)
    }
}
