# dropclock

[![CI](https://github.com/JayKickliter/dropclock/actions/workflows/ci.yml/badge.svg)](https://github.com/JayKickliter/dropclock/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/dropclock/badge.svg)](https://docs.rs/dropclock)
[![main docs](https://img.shields.io/badge/docs-main-blue)](https://jaykickliter.github.io/dropclock/dropclock/)

This crate provides `DropClock`, a guard type that captures
`std::time::Instant::now()` on creation and, when dropped, passes that
instant to a user-provided closure. Useful for timing scopes, logging
elapsed durations, or recording metrics without manual bookkeeping.

```rust
use dropclock::DropClock;

let _timer = DropClock::new(|start| {
    eprintln!("elapsed: {:?}", start.elapsed());
});
// ... your code here ...
// closure fires when _timer is dropped
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT License](LICENSE-MIT) at your option.
