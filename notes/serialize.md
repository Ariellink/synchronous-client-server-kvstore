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
测试`serde_json::from_reader()`和 `Point::deserialize(&mut Deserializer::from_reader())`两种反序列化方式的区别。
```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::error> 
    where
        D: Deserializer<'de>
}
```
在大多数时候，serde derive方法都能生成自定义struct和enum的`Deserialize`的实现。

### derive Deserialize and Serialize for structs

You only need to set this up if your code is using `#[derive(Serialize, Deserialize)]`.

```rust
use std::fs::File;
use std::{string, io::BufReader};
use serde::{Deserialize,Serialize};
use std::io::{Read, Write, BufWriter};
use std::fs;
use serde_json::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: String,
    y: i32,
}

fn main() -> Result<(), std::io::Error> {
    {
        let point = Point {x:"abc".to_string(),y: 234};

        let serialize_point = serde_json::to_vec(&point).unwrap();
        let iofile = File::create("/home/chenxi0912/rusttest/projects/feature_tests/serde/0403.txt").unwrap();
        let mut writer = std::io::BufWriter::new(iofile);
        writer.write_all(&serialize_point)?;
        
        println!("serialized = {:?}", serialize_point);
    }
    // 测试serde_json::from_reader()反序列化： https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html
    // pub fn from_reader<R, T>(rdr: R) -> Result<T>where
    // R: Read,
    // T: DeserializeOwned,
    {
        let deserialized_point: Point = serde_json::from_reader(BufReader::new(File::open("/home/chenxi0912/rusttest/projects/feature_tests/serde/0403.txt").unwrap())).unwrap();
        println!("deserialized = {:#?}", deserialized_point);
    }
    //测试custom_struct::deserialize()反序列化
    //trait serde::de::Deserialize 通过derive的方式被自定义struct Point继承
    //那么自动获得Point::deserialize这个接口（required method）
    /*
    pub trait Deserialize<'de>: Sized {
    // Required method
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
       where D: Deserializer<'de>;
    */
    //由于参数deserializer必须要impl Trait serde::Deserializer这个trait, 因此这里somehow使用了Struct serde_json::Deserializer<R: Read>从bufread中构造了一个deserializer作为参数。
    {
        let deserialized_point2 = Point::deserialize(&mut Deserializer::from_reader(BufReader::new(File::open("/home/chenxi0912/rusttest/projects/feature_tests/serde/0403.txt").unwrap()))).unwrap();
        println!("deserialized = {:#?}", deserialized_point2);
    }
    Ok(())
}
```
