//#![deny(missing_docs)]
/*!
The KvStore store key/value pairs.
*/

mod errors;
mod engine;
mod request;

pub use errors::{KVStoreError, Result};
//pub use kv::KvStore;
pub use engine::KvsEngine;
pub use engine::Command;
pub use engine::KvStore;
pub use request::Request;