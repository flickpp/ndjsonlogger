
# ndjsonlogger

ndjsonlogger is an nd (newline delimited) json logger.

## Quickstart

```rust
use ndjsonlogger::info;

fn main() {
    ndjsonlogger_init(None, false);
    info!("hello I'm a log line");
}}
```

```json
{"level": "info", "msg": "hello I'm a log line"}
```

## Contributing

Contributions welcome. Please open a github issue.
