## how logging works in Rust?

(https://docs.rs/env_logger/0.10.0/env_logger/index.html)

A log request consists of a *target*, a *level*, and a *body*.

### Usage
five logging macros: `error!`, `warn!`, `info!`, `debug!` and `trace!`.

log crate exposes the logging facade.

#### Logger
In order to produce log output executables have to use a logger implementation compatible with the facade.   
Loggers implement the `Log` trait.  

There are many available implementations to choose fromlogger can be configured via environment variables

##### Available logger
`env_logger` writes logs to stderr, you can configure it to write the log to stdout.
Log level is configured by environment variables. By default all logging is disabled except for the error level.

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

