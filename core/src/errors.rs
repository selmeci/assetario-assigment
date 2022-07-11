use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetarioError {
    #[error("Cannot download data from simple maps")]
    CannotDownloadData,
    #[error("ZIP file is invalid")]
    InvalidZipFile,
    #[error("ZIP file cannot be created")]
    CannotCreateZipFile,
    #[error("Invalid data in stream")]
    InvalidDataInStream,
    #[error("Cannot write to zip file")]
    CannotWriteToZip,
    #[error("Cannot read from zip file")]
    CannotReadFromZip,
    #[error("Cache error")]
    CacheError,
    #[error("DynamoDB error")]
    DynamoDBError,
}
