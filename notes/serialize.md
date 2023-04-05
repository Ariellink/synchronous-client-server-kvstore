https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html
Deserializer::from_reader

Request::deserialize(&mut Deserializer::from_reader(BufReader::new(&mut stream)))?;

struct Client {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

        match Response::deserialize(&mut self.reader)? {
            Response::Ok(value) => Ok(value),
            Response::Err(err) => Err(KVStoreError::CommonStringError(err)),
        }

```rust
use serde_json::Deserializer;
serde_json::to_vec() 序列化为vec
serde_json::from_reader() 反序列化
```

## Trait serde::Deserialize 

```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::error> 
    where
        D: Deserializer<'de>
}
```
在大多数时候，serde derive方法都能生成自定义struct和enum的`Deserialize`的实现。

## derive Deserialize and Serialize for structs

You only need to set this up if your code is using `#[derive(Serialize, Deserialize)]`.

