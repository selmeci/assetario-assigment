use anyhow::{anyhow, Result};
use s3::creds::Credentials;
use s3::{Bucket, Region};

use crate::errors::AssetarioError;
use crate::structures::GqlTree;

pub struct Cache;

impl Cache {
    fn path<I: AsRef<str>>(id: I) -> String {
        format!("demo/assetario/cache/{}.json", id.as_ref())
    }

    fn bucket() -> Result<Bucket> {
        Bucket::new(
            "selma-solutions-assetario",
            Region::EuCentral1,
            Credentials::from_profile(None)
                .or_else(|_| Credentials::from_env())
                .expect("AWS credentials"),
        )
        .map_err(|err| {
            println!("AWS S3 bucket error. err {:?}", err);
            anyhow!(AssetarioError::CacheError)
        })
    }

    pub async fn store<I: AsRef<str>>(id: I, tree: &GqlTree) -> Result<()> {
        println!("Store tree in cache. {}", id.as_ref());
        let json = serde_json::to_vec_pretty(&tree).expect("JSON");
        let bucket = Self::bucket()?;
        let (_, code) = bucket
            .put_object_with_content_type(Self::path(id), &json, "application/json")
            .await
            .map_err(|err| {
                println!("AWS S3 bucket upload error. err {:?}", err);
                anyhow!(AssetarioError::CacheError)
            })?;
        if code != 200 {
            return Err(anyhow!(AssetarioError::CacheError));
        }
        Ok(())
    }

    pub async fn get<I: AsRef<str>>(id: I) -> Result<Option<GqlTree>> {
        let bucket = Self::bucket()?;
        let (content, code) = bucket.get_object(Self::path(id)).await.map_err(|err| {
            println!("AWS S3 bucket get error. err {:?}", err);
            anyhow!(AssetarioError::CacheError)
        })?;
        if code == 200 {
            Ok(serde_json::from_slice(content.as_slice()).expect("JSON"))
        } else {
            Ok(None)
        }
    }
}
