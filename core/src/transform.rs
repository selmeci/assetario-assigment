use anyhow::Result;
use futures_core::Stream;
use futures_util::{pin_mut, StreamExt};
use maplit::btreemap;
use std::sync::Arc;

use crate::structures::{City, Country, GqlTree, State, Tree, TreeCity};

pub struct Transformer;

impl Transformer {
    ///
    /// build tree state -> country -> city with BTreeMap -> O(n)
    ///
    pub async fn transform(cities: impl Stream<Item = City>) -> Result<Tree> {
        pin_mut!(cities);
        let tree = cities
            //reduce memory footprint
            .map(|city| TreeCity::from(city))
            // build tree
            .fold(Tree::new(), |mut tmp, city| async move {
                if let Some(countries) = tmp.get_mut(&city.state) {
                    if let Some(cities) = countries.get_mut(&city.county) {
                        cities.push(city);
                    } else {
                        countries.insert(Arc::clone(&city.county), vec![city]);
                    }
                } else {
                    tmp.insert(
                        Arc::clone(&city.state),
                        btreemap! {Arc::clone(&city.county) => vec![city]},
                    );
                }
                tmp
            })
            .await;
        Ok(tree)
    }

    ///
    /// Gql does not allow unknown attributes in object -> transform hash map to vec of vectors
    ///
    pub async fn gql_tree(tree: Tree) -> Result<GqlTree> {
        Ok(tree
            .into_iter()
            .fold(Vec::new(), |mut tmp, (state, countries)| {
                tmp.push(State {
                    name: state.to_string(),
                    countries: countries
                        .into_iter()
                        .map(|(country, mut cities)| {
                            cities.sort_by(|a, b| a.name.cmp(&b.name));
                            Country {
                                name: country.to_string(),
                                cities: cities
                                    .into_iter()
                                    .map(|city| City {
                                        id: city.id,
                                        name: city.name.to_string(),
                                        state: city.state.to_string(),
                                        country: city.county.to_string(),
                                    })
                                    .collect(),
                            }
                        })
                        .collect(),
                });
                tmp
            }))
    }
}
