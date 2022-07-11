use core::cache::Cache;
use core::extract::Extractor;
use core::load::Loader;
use core::transform::Transformer;

use futures_util::TryFutureExt;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    println!("Starting ETL");
    match Extractor::extract()
        .and_then(Loader::load)
        .and_then(Transformer::transform)
        .and_then(Transformer::gql_tree)
        .and_then(|tree| async move {
            Cache::store("tree", &tree).await?;
            Ok(tree)
        })
        .await
    {
        Ok(tree) => println!("{:#?}", tree),
        Err(err) => {
            println!("{:?}", err)
        }
    };
}
