use maplit::hashmap;
use rusoto_dynamodb::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub type Tree = BTreeMap<Arc<String>, BTreeMap<Arc<String>, Vec<TreeCity>>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub name: String,
    pub countries: Vec<Country>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Country {
    pub name: String,
    pub cities: Vec<City>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct City {
    pub id: String,
    #[serde(alias = "city", alias = "name")]
    pub name: String,
    #[serde(alias = "state_name", alias = "state")]
    pub state: String,
    #[serde(alias = "county_name", alias = "country")]
    pub country: String,
}

pub type GqlTree = Vec<State>;

impl Into<HashMap<String, AttributeValue>> for City {
    fn into(self) -> HashMap<String, AttributeValue> {
        hashmap! {
            "PK".to_string() => AttributeValue {
                s: Some(format!("city#id#{}",self.id.clone())),
                ..AttributeValue::default()
            },
            "SK".to_string() => AttributeValue {
                s: Some(format!("city#id#{}",self.id.clone())),
                ..AttributeValue::default()
            },
            "G1PK".to_string() => AttributeValue {
                s: Some(format!("state#{}",self.state)),
                ..AttributeValue::default()
            },
            "G1SK".to_string() => AttributeValue {
                s: Some(format!("country#{}",self.country)),
                ..AttributeValue::default()
            },
            "G2PK".to_string() => AttributeValue {
                s: Some("cityByName".to_string()),
                ..AttributeValue::default()
            },
            "G2SK".to_string() => AttributeValue {
                s: Some(format!("city#{}",self.name)),
                ..AttributeValue::default()
            },
            "id".to_string() => AttributeValue {
                s: Some(self.id),
                ..AttributeValue::default()
            },
            "name".to_string() => AttributeValue {
                s: Some(self.name),
                ..AttributeValue::default()
            },
            "state".to_string() => AttributeValue {
                s: Some(self.state),
                ..AttributeValue::default()
            },
            "country".to_string() => AttributeValue {
                s: Some(self.country),
                ..AttributeValue::default()
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TreeCity {
    pub id: String,
    pub name: Arc<String>,
    pub state: Arc<String>,
    pub county: Arc<String>,
}

impl From<City> for TreeCity {
    fn from(other: City) -> Self {
        Self {
            id: other.id,
            name: Arc::new(other.name),
            state: Arc::new(other.state),
            county: Arc::new(other.country),
        }
    }
}
