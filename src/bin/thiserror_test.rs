use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn main() {
    let err = DataStoreError::Disconnect(io::Error::other("oh no"));
    println!("{:?}", err);
    let err = DataStoreError::Redaction("foo".to_string());
    println!("{:?}", err);
    let err = DataStoreError::InvalidHeader {
        expected: "foo".to_string(),
        found: "bar".to_string(),
    };
    println!("{:?}", err);
    let err = DataStoreError::Unknown;
    println!("{:?}", err);
}
