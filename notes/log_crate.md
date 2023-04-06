# Logging

- [how logging works in Rust?](https://github.com/Ariellink/synchronous-client-server-kvstore/blob/main/notes/log_crate.md#how-logging-works-in-rust)
    - log crate
    - logger
        - env_logger usage
- [kvs-server add logging](https://github.com/Ariellink/synchronous-client-server-kvstore/blob/main/notes/log_crate.md#kvs-server-add-logging) 

# how logging works in Rust?
log crate: (https://docs.rs/log/latest/log/)  
env_logger: (https://docs.rs/env_logger/0.10.0/env_logger/index.html)

A log request consists of a *target*, a *level*, and a *body*.
### how logging works in Rust？

#### Logging facade:  ->`log crate` (only provides logging apis)  
-> libs (such as `env_logger`) to impl these logging apis in their logging impplementations(namely `logger`)   
-> users choose these logging libs for their use case (https://docs.rs/log/latest/log/#available-logging-implementations) or write their own loggers (https://docs.rs/log/latest/log/#implementing-a-logger)

### Usage
five logging macros: `error!`, `warn!`, `info!`, `debug!` and `trace!`.

log crate exposes the logging facade. 

`Log` trait source code:
```rust
/// A trait encapsulating the operations required of a logger.
pub trait Log: Sync + Send {
    /// Determines if a log message with the specified metadata would be
    /// logged.
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// discarded anyway.
    ///
    /// # For implementors
    ///
    /// This method isn't called automatically by the `log!` macros.
    /// It's up to an implementation of the `Log` trait to call `enabled` in its own
    /// `log` method implementation to guarantee that filtering is applied.
    fn enabled(&self, metadata: &Metadata) -> bool;

    /// Logs the `Record`.
    ///
    /// # For implementors
    ///
    /// Note that `enabled` is *not* necessarily called before this method.
    /// Implementations of `log` should perform all necessary filtering
    /// internally.
    fn log(&self, record: &Record);

    /// Flushes any buffered records.
    fn flush(&self);
}

```

#### Logger
In order to produce log output executables have to use a logger implementation compatible with the facade.   
Loggers implement the `Log` trait.  

There are many available implementations to choose from logger can be configured via environment variables

##### Available logger

env_logger | simple_logger | simplelog | ...  


`env_logger` writes logs to stderr（默认情况下）, you can configure it to write the log to stdout.
Log level is configured by environment variables. By default all logging is disabled except for the error level.

### 如何构造一个env_logger::Logger实例？
- 🚩`Struct env_logger::Logger`: impl `Log` trait from the log crate, which allows it to act as a logger. The `init(), try_init(), Builder::init() and Builder::try_init()` methods will each construct a Logger and immediately initialize it as the default global logger.
- `Function env_logger::init`: Initializes the global logger with an env logger.   
- `Struct env_logger::Builder`: It can be used to customize the log format, change the environment variable used to provide the logging directives and also set the default log level filter.

```Rust
use log::{debug, error, log_enabled, info, Level};

fn main() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}

// cmd: RUST_LOG=error ./target/debug/log_api (default)
// cmd: RUST_LOG=info cargo run (execute line 11 - 14)
// cmd: RUST_LOG=info ./target/debug/log_api (execute line 11 - 14)
// cmd: RUST_LOG=debug ./target/debug/log_api (execute all lines, including error and info)
```

```toml
[dependencies]
log = "0.4.17"
env_logger = "0.10.0"
```
Logging is controlled via the `RUST_LOG` environment variable。

output:
```sh
➜ RUST_LOG=debug ./target/debug/log_api
[2023-02-26T03:34:30Z DEBUG log_api] this is a debug message
[2023-02-26T03:34:30Z ERROR log_api] this is printed by default
[2023-02-26T03:34:30Z INFO  log_api] the answer was: 12

```

 # kvs-server add logging
 
 1. add log crate as dependencies
    ```toml
    [dependencies]
    log = { version = "0.4.17", features = ["std", "serde"] }
    env_logger = "0.10.0"
    ```
 2. modify kvs-server to initialize logging on startup, prior to command-line parsing
    - On startup log the server's version number. Also log the configuration. For now that means the IP address and port, and the name of the storage engine.  
    *kvs-server.rs*
    ```rust
    fn main() -> Result<()> {
    //logger 
    env_logger::builder().filter_level(LevelFilter::Info).init();
    ```
    📌`Function env_logger::builder`: create a new builder with the default environment variables.  
    
    ```rust
    pub fn builder() -> Builder
    ```
   📌 `Struct env_logger::Builder`: builder acts as builder for initializing a Logger. For customizing the *log format, change the environment variable used to provide the logging directives and also set the default log level filter.   
   - `pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self`向所有模块的filter添加指令directives
    ```rust
    use env_logger::Builder;
    use log::LevelFilter;

    let mut builder = Builder::new();

    builder.filter_level(LevelFilter::Info);
    ```
      
    - server's version number ▶️ https://doc.rust-lang.org/cargo/reference/environment-variables.html   
    ```rust
    let version = env!("CARGO_PKG_VERSION");
    ```
    - confoguration: ipaddress and port 
    ```rust
    fn init(matches: ArgMatches) -> Result<()> {

    let addr = matches.get_one::<String>
    ("addr").unwrap();
    let engine_type_userspecified = matches.get_one::<String>
    ("engine").unwrap();

    //logger
    info!("Version: {}",env!("CARGO_PKG_VERSION"));
    info!("Addr: [{}]", addr);
    info!("EngineTypeSpecifiedByUser: [{}]", engine_type_userspecified);
    ```
    - record the request server received and response made before sending to client `info!("Request: {:?}", &request);` and `info!("Response: {:?}", &response);`
    *server.rs*
    ```rust
    use log::info;
    
    impl <E: KvsEngine> KvServer<E> {
       fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()> {
         //序列化request
         //let a = Request::deserialize(&mut serde_json::Deserializer::new(BufReader::new(&mut stream)))?;
         //@proticol.md::Response
         let request:Request = serde_json::from_reader(BufReader::new(&mut stream))?;

        info!("Request: {:?}", &request);

         let response;
         match request {
            Request::GET(key) => {
                match self.engine.get(key) {
                    Ok(value) => response = Response::Ok(value),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
            Request::SET(key, val) => {
                match self.engine.set(key, val) {
                    Ok(()) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
            Request::RM(key) => {
                match self.engine.remove(key) {
                    Ok(()) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
         }
        
        info!("Response: {:?}", &response);
        
        serde_json::to_writer(stream, &response)?;
        
        Ok(())
    }
    ```
 4. Set it up to output to stderr (sending the logs elsewhere additionally is fine, but they must go to stderr to pass the tests in this project)

