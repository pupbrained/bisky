use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record<T> {
    pub uri: String,
    pub cid: String,
    pub value: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListRecordsOutput<T> {
    pub cursor: Option<String>,
    pub records: Vec<Record<T>>,
}

#[derive(Serialize)]
pub struct CreateRecord<'a, T> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub record: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRecordOutput {
    pub cid: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUploadBlob {
    pub blob: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename(deserialize = "$link", serialize = "$link"))]
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Blob {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    pub r#ref: Link,
    #[serde(rename(deserialize = "mimeType", serialize = "mimeType"))]
    pub mime_type: String,
    pub size: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlobOutput {
    pub blob: Blob,
}
