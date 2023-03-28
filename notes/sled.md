# Sled

[sled doc](https://docs.rs/sled/latest/sled/index.html)
a embedded kv db.  
需要把sled嵌入到kvstore中成为另一个可以切换的存储引擎。

*Cargo.toml*
```toml
[dependencies]
sled = "0.34.7"
```

### Example
```rust
let db: sled::Db = sled::open("my_db").unwrap();
```
找到`sled`的`Db Struct`▶️ [Struct sled::Db](https://docs.rs/sled/latest/sled/struct.Db.html) 通过open来构造

```rust
pub fn open<P: AsRef<Path>>(path: P) -> Result<Db>
```
`sled::open`: opens a Db with a default configuration at the specific path. 

## SET方法
https://docs.rs/sled/latest/sled/struct.Db.html#method.insert
```rust
pub fn insert<K, V>(&self, key: K, value: V) -> Result<Option<IVec>> where
    K: AsRef<[u8]>,
    V: Into<IVec>, 
//Insert a key to a new value, returning the last value if it was set.
//example:
assert_eq!(db.insert(&[1, 2, 3], vec![0]), Ok(None));
assert_eq!(db.insert(&[1, 2, 3], vec![1]), Ok(Some(sled::IVec::from(&[0]))));
```
## GET方法
https://docs.rs/sled/latest/sled/struct.Db.html#method.get 
```rust
pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>>
//Retrieve a value from the Tree if it exists.
//example:
db.insert(&[0], vec![0])?;
assert_eq!(db.get(&[0]), Ok(Some(sled::IVec::from(vec![0]))));
assert_eq!(db.get(&[1]), Ok(None));
```

## RM 方法
https://docs.rs/sled/latest/sled/struct.Db.html#method.remove
```rust
pub fn remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>>

//Delete a value, returning the old value if it existed.
//example:
db.insert(&[1], vec![1]);
assert_eq!(db.remove(&[1]), Ok(Some(sled::IVec::from(vec![1]))));
assert_eq!(db.remove(&[1]), Ok(None));
```

## Enum sled::Error
https://docs.rs/sled/latest/sled/enum.Error.html

```rust
pub enum Error {
    CollectionNotFound(IVec),
    Unsupported(String),
    ReportableBug(String),
    Io(Error),
    Corruption {
        at: Option<DiskPtr>,
        bt: (),
    },
}
```

*In errors.rs*
```rust
use failure::Fail;

pub type Result<T> = std::result::Result<T,KVStoreError>;

#[derive(Fail, Debug)]
pub enum KVStoreError {
    ...
    //(4) merge Error from sled::Error
    #[fail(display = "{}", _0)]
    SledError(#[cause] sled::Error),
}

impl From<sled::Error> for KVStoreError {
    fn from(err: sled::Error) -> Self {
        KVStoreError::SledError(err)
    }
}

```

>📌`use crate::{KvsEngine,KVStoreError,Result};`
Result必须被引用进来。  

*lib.rs*
```rust
mod errors;
pub use errors::{KVStoreError, Result};
```
*In errors.rs* **Result和KVStoreError同级导出。**
```rust

pub type Result<T> = std::result::Result<T,KVStoreError>;

#[derive(Fail, Debug)]
pub enum KVStoreError {
```

## Db::flush()  
https://docs.rs/sled/latest/sled/struct.Db.html#method.flush 