# ndjsonlogger

ndjsonlogger is an nd (newline delimited) json logger.

Rust 1.60 or greater is required.

## TODO/Features

- [x] debug, info, warn and error macros
- [x] debug! macro compiles to no-op on release builds
- [x] all JSON primative types (number, bool, null) supported
- [x] one level of nested arrays
- [x] compile-time iso timestamp feature
- [x] quickstart print log lines to stdout
- [ ] configurable alternative sinks for log lines
- [ ] initialize with service name - add to all log lines
- [ ] trace macro with trace mask
- [ ] custom runtime logic for additional key/value(s)
- [ ] work with ndjsonloggercore `no_std`


## Quickstart

```toml
[dependencies]
ndjsonlogger = "0.1"
ndjsonloggercore = {version = "0.1", features = ["std"]}
```

NOTE: You must include BOTH lines in your Cargo.toml.
Additionally for the 0.1 release, the std feature is required in ndjsonloggercore.

```rust
use ndjsonlogger::{info, debug};

fn main() {
    info!("hello I'm a log line");

    debug!("application closing", {
        reason = "end of main function"
    });
}}
```

```json
{"level": "info", "msg": "hello I'm a log line"}
{"level": "debug", "msg": "application closing", "reason": "end of main function"}
```

An example demonstrating all features is [here](../master/example/src/main.rs).

## Contributing

Contributions welcome. Please open a github issue.
