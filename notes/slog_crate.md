# How slogs works in Rust?

Drain, Logger and log macro are the most important elements of slog.  
https://docs.rs/slog/latest/slog/trait.Drain.html

### Example to logging to a file
```rust
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use std::fs::OpenOptions;
use slog::Drain;

fn main() {
   let log_path = "target/your_log_file_path.log";
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());
}
```

Typically the biggest problem is creating a Drain

### Drain tarit
`Darin` write the logs into given destination(s). [logging to the terminal; to file; changing logging level at runtime]     

#### Trait Required Methods
``` rust
pub fn log(
    &self,
    record: &Record<'_>,
    values: &OwnedKVList
) -> Result<Self::Ok, Self::Err>
```
#### Any custom log handling logic should be implemented as a `Drain`.
##### Trait implement Example: Change logging level at runtime (Custom Drain logic)
```rust
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;

use std::sync::{Arc, atomic};
use std::sync::atomic::Ordering;
use std::result;

/// Custom Drain logic
struct RuntimeLevelFilter<D>{
   drain: D,
   on: Arc<atomic::AtomicBool>,
}

impl<D> Drain for RuntimeLevelFilter<D>
    where D : Drain {
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(&self,
          record: &slog::Record,
          values: &slog::OwnedKVList)
          -> result::Result<Self::Ok, Self::Err> {
          let current_level = if self.on.load(Ordering::Relaxed) {
              slog::Level::Trace
          } else {
              slog::Level::Info
          };

          if record.level().is_at_least(current_level) {
              self.drain.log(
                  record,
                  values
              )
              .map(Some)
              .map_err(Some)
          } else {
              Ok(None)
          }
      }
  }

fn main() {
    // atomic variable controlling logging level
    let on = Arc::new(atomic::AtomicBool::new(false));

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build();
    let drain = RuntimeLevelFilter {
        drain: drain,
        on: on.clone(),
    }.fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    // switch level in your code
    on.store(true, Ordering::Relaxed);
}

```
