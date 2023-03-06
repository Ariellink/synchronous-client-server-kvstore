# kvs-server

## trait KvsEngine
为了扩展存储引擎的多种实现，抽象出来了统一的 trait 接口 KvsEngine 以对上暴露 trait 的抽象而隐藏具体的实现细节。这样 kvs-server 在启动时便可以以 trait 的方式去访问 engine，而不需要在意其内部的实现细节。

根据要求，实现`trait KvsEngine`.

在engine/kvs_engine.rs定义trait:

```rust
use crate::Result; //type in error.rs

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
  }
```
## kv.rs 
对于 KvStore，将其 set/get/remove 这三个方法抽象到了 KvsEngine 的实现中。
### KvStore 结构体成员
```rust
struct CommandPos {
    offset: u64,
    length: u64,
    file_id: u64,
}

pub struct KvStore {
    // key：String， vaule_metadata: CommandPos
    index: HashMap<String, CommandPos>,
    current_reader: HashMap<u64,BufReader<File>>,
    current_writer: BufWriterWithPos<File>,
    current_file_id: u64,
    dir_path: PathBuf,
    size_for_compaction: u64,
}
```
#### KvStore特有methods
```rust
impl KvStore {
  pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {}
  fn create_new_file(&mut self) -> Result<()> {}
  fn recover()
}
```
#### KvStore 继承 `KvsEngine`trait 中的required methods
> 在project 2 中 kv.rs的基础上, 为`KvStore`实现`KvsEngine`trait.

```rust
impl KvsEngine for KvStore {
  fn set(&mut self, key: String, value: String) -> Result<()> {}
  fn get(&mut self, key: String) -> Result<Option<String>> {}
  fn remove(&mut self, key: String) -> Result<()> {}
}
```

### struct BufWriterWithPos
```rust
  struct BufWriterWithPos<T>
  where
      T : Write + Seek
  {
      bufwriter: BufWriter<T>,
      position: u64,
}
```
#### implementations for BufWriterWithPos

```rust
use crate::KVStoreError;
use crate::Result;
//construction and locate func definition 
impl <T: Write + Seek> BufWriterWithPos<T> {
    //inherate the KVStoreError
    fn new(mut inner: T) -> Result<Self> {
        Ok(
            BufWriterWithPos {  
                //move the cursor 0 byte from the end of file
                //return the cursor postions Result<u64>
                position: inner.seek(SeekFrom::End(0))?,
                //create the writer buffer using T
                bufwriter: BufWriter::new(inner), 
            }
        )
    }

    fn get_position(&self) -> u64 {
        self.position
    }
}

//impl Writer trait for BufWriterWithPos so that it can use Writer's methods defined in std::io and fs lib
use crate::command::Command;
impl <T: Write + Seek> Write for BufWriterWithPos<T> {
    
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.bufwriter.write(buf)?; // return how many bytes written
        self.position += len as u64; //usize to u64
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.bufwriter.flush()
    }

```
## sled.rs
对于 Sled，同样实现了 KvsEngine 的三个方法。需要注意其默认接口的语义和格式与 KvsEngine 不一致，因而需要增加对应的转换。
为`sled`实现`KvsEngine trait`
```rust

```