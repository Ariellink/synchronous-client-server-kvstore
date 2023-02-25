use failure::Fail;

pub type Result<T> = std::result::Result<T,KVStoreError>;

#[derive(Fail, Debug)]
pub enum KVStoreError {
    //(1) merge Error from std::io
    #[fail(display = "{}", _0)]
    IoError(#[cause] std::io::Error),

    //(2) merge Error from serde_json
    #[fail(display = "{}", _0)]
    SerdeError(#[cause] serde_json::Error),

    #[fail(display = "Unknown command type")]
    UnknownCommandType,

    #[fail(display = "Key not found")]
    KeyNotFound,

}

impl From<std::io::Error> for KVStoreError {
    fn from(err: std::io::Error) -> Self {
        KVStoreError::IoError(err)
    }

}

impl From<serde_json::Error> for KVStoreError {
    fn from(err: serde_json::Error) -> Self {
        KVStoreError::SerdeError(err)
    }
}