## how logging works in Rust?

(https://docs.rs/env_logger/0.10.0/env_logger/index.html)

A log request consists of a *target*, a *level*, and a *body*.

### Usage
five logging macros: `error!`, `warn!`, `info!`, `debug!` and `trace!`.

log crate exposed the logging facade.

#### Logger
logger can be configured via environment variables

##### Available logger
`env_logger` writes logs to stderr, you can configure it to write the log to stdout.
Log level is configured by environment variables. By default all logging is disabled except for the error level.

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

