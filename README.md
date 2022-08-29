cfg_log
=======

Compile time conditional logging.

[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/cfg_log&color=brightgreen&logo=github)](https://github.com/FedericoStra/cfg_log)
[![Crates.io](https://img.shields.io/crates/v/cfg_log?logo=rust)](https://crates.io/crates/cfg_log)
[![docs.rs](https://img.shields.io/docsrs/cfg_log?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBkPSJNNDg4LjYgMjUwLjJMMzkyIDIxNFYxMDUuNWMwLTE1LTkuMy0yOC40LTIzLjQtMzMuN2wtMTAwLTM3LjVjLTguMS0zLjEtMTcuMS0zLjEtMjUuMyAwbC0xMDAgMzcuNWMtMTQuMSA1LjMtMjMuNCAxOC43LTIzLjQgMzMuN1YyMTRsLTk2LjYgMzYuMkM5LjMgMjU1LjUgMCAyNjguOSAwIDI4My45VjM5NGMwIDEzLjYgNy43IDI2LjEgMTkuOSAzMi4ybDEwMCA1MGMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAzLjktNTIgMTAzLjkgNTJjMTAuMSA1LjEgMjIuMSA1LjEgMzIuMiAwbDEwMC01MGMxMi4yLTYuMSAxOS45LTE4LjYgMTkuOS0zMi4yVjI4My45YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43ek0zNTggMjE0LjhsLTg1IDMxLjl2LTY4LjJsODUtMzd2NzMuM3pNMTU0IDEwNC4xbDEwMi0zOC4yIDEwMiAzOC4ydi42bC0xMDIgNDEuNC0xMDItNDEuNHYtLjZ6bTg0IDI5MS4xbC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnptMjQwIDExMmwtODUgNDIuNXYtNzkuMWw4NS0zOC44djc1LjR6bTAtMTEybC0xMDIgNDEuNC0xMDItNDEuNHYtLjZsMTAyLTM4LjIgMTAyIDM4LjJ2LjZ6IiBzdHlsZT0iZmlsbDojZmZmZmZmIj48L3BhdGg+PC9zdmc+Cg==)](https://docs.rs/cfg_log)
[![MIT license](https://img.shields.io/crates/l/cfg_log)](https://github.com/FedericoStra/cfg_log/blob/master/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/FedericoStra/cfg_log/Rust)](https://github.com/FedericoStra/cfg_log/actions/workflows/rust.yml)
![Lines of code](https://tokei.rs/b1/github/FedericoStra/cfg_log?category=code)

## Usage

The main crate should depend on `cfg_log` and optionally on `log`.

```toml
[dependencies]
cfg_log = "0.1.0"
log = { version = "0.4.17", optional = true }
```

Then logging can be done more concisely with

```rust
use cfg_log::*;

fn main() {
    debug!("the answer is {}", 42);
}
```

instead of

```rust
#[cfg(feature = "log")]
use log::*;

fn main() {
    #[cfg(feature = "log")]
    debug!("the answer is {}", 42);
}
```

The `debug!` macro will automatically expand to `log::debug!` if the `log` feature is enabled,
or it will be discarded at compile time otherwise.
