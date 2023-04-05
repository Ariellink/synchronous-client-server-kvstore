# synchronous-client-server-kvstore
A single-threaded, persistent key/value store server and client with synchronous networking over a custom protocol.

### Status
- [x] log crate & slog crate
- [x] kvs-client: lib  + main  : The `kvs-client` binary accepts the same command line arguments as in previous projects. 
- [x] kvs-server: 重写参数解析器 : `kvs-server` has its own set of command line arguments to handle, as described previously in the spec.
  - [x] kvsEngine tarit 
- [x] protocol: pending design  - 命令传输
  - [x] part 3 Client-server networking
  - [x] part 4 Implement command across the network
- [ ] 日志打印
- [x] 可扩展存储引擎 `KvsEngine`, `SledKvsEngine`
- [ ] benchmark 性能测试

### kvs lib  
https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-3/README.md#user-content-project-setup 
- KvsClient - implements the functionality required for kvs-client to speak to kvs-server
- KvsServer - implements the functionality to serve responses to kvs-client from kvs-server
- `KvsEngine` trait - defines the storage interface called by KvsServer
- KvStore - implements by hand the `KvsEngine trait
- SledKvsEngine - implements `KvsEngine` for the sled storage engine.

## trait `KvsEngine` requeired methods

```rust
pub trait KvsEngine {
  KvsEngine::set(&mut self, key: String, value: String) -> Result<()>;
  KvsEngine::get(&mut self, key: String) -> Result<Option<String>>;
  KvsEngine::remove(&mut self, key: String) -> Result<()>;
}

```
