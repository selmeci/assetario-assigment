use anyhow::{anyhow, Result};
use async_stream::stream;
use async_zip::read::seek::ZipFileReader;
use csv_async::AsyncReaderBuilder;
use futures_core::Stream;
use futures_util::io::Cursor;
use futures_util::{StreamExt, TryFutureExt};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::errors::AssetarioError;
use crate::structures::City;

const SIMPLE_MAPS_ZIP_PATH: &str = "/tmp/simplemaps.zip";

#[derive(Default)]
pub struct Extractor;

impl Extractor {
    ///
    /// Download file from AWS S3, because original site is protected with Captcha
    ///
    async fn download_zip() -> Result<File> {
        println!("Downloading zip file from web");
        // closure release file after download
        let mut stream = reqwest::get(
            "https://selma-solutions-assetario.s3.eu-central-1.amazonaws.com/demo/assetario/simplemaps.zip",
        )
        .await
        .map_err(|_| AssetarioError::CannotDownloadData)?
        .bytes_stream();
        let mut zip = File::create(SIMPLE_MAPS_ZIP_PATH)
            .await
            .map_err(|_| AssetarioError::CannotCreateZipFile)?;
        let mut progress = 0;
        while let Some(item) = stream.next().await {
            if let Ok(chunk) = item {
                progress += chunk.len();
                println!("Downloading: \t{}", progress);
                zip.write(&chunk)
                    .map_err(|_| AssetarioError::CannotWriteToZip)
                    .await?;
            } else {
                return Err(anyhow!(AssetarioError::InvalidDataInStream));
            }
        }
        println!("Zip downloaded.");
        File::open(SIMPLE_MAPS_ZIP_PATH)
            .await
            .map_err(|_| anyhow!(AssetarioError::CannotReadFromZip))
    }

    ///
    /// Read content of "uscities.csv" from zip
    ///
    async fn unzip_csv(file: File) -> Result<Vec<u8>> {
        println!("Unzip CSV file");
        let mut zip = ZipFileReader::new(file).await.unwrap();
        if let Some((index, _)) = zip.entry("uscities.csv") {
            zip.entry_reader(index)
                .await
                .map_err(|err| {
                    println!("Zip error: {:?}", err);
                    anyhow!(AssetarioError::InvalidZipFile)
                })?
                .read_to_end_crc()
                .await
                .map_err(|err| {
                    println!("Zip error. {:?}", err);
                    anyhow!(AssetarioError::InvalidZipFile)
                })
        } else {
            // zip always contains uscities.csv
            unreachable!()
        }
    }

    ///
    /// Deserialize text into structure
    ///
    async fn parse_csv(csv: Vec<u8>) -> Result<impl Stream<Item = City>> {
        println!("Parse CSV file");
        Ok(stream! {
            let rdr = AsyncReaderBuilder::new().create_deserializer(Cursor::new(csv));
            let mut csv = rdr.into_deserialize::<City>();
            while let Some(city) = csv.next().await {
                if let Ok(city) = city {
                    yield city;
                } else {
                    println!("Invalid csv data for city")
                }
            }
        })
    }

    ///
    /// Get extract data
    ///
    pub async fn extract() -> Result<impl Stream<Item = City>> {
        File::open(SIMPLE_MAPS_ZIP_PATH)
            // download zip if not cached
            .or_else(|_| Self::download_zip())
            // unzip file and return csv text
            .and_then(Self::unzip_csv)
            // parse csv into city structure
            .and_then(Self::parse_csv)
            .await
    }
}
