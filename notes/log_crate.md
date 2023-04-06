# Logging

- [kvs-server add logging] (# kvs-server add logging)

## how logging works in Rust?
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
 
 
