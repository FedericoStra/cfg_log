//! Compile time conditional logging.
//!
//! The `cfg_log` crate provides a way to conditionally use the [logging macros] from the
//! `log` crate depending on whether the `log` feature of the main crate is enabled or not.
//!
//!   [logging macros]: https://docs.rs/log/latest/log/#macros
//!
//! Suppose the `Cargo.toml` file of a crate using `cfg_log` contains
//!
//! ```toml
//! [dependencies]
//! cfg_log = "0.1.0"
//! log = { version = "0.4.17", optional = true }
//! ```
//!
//! Then the code
//!
//! ```rust
//! use cfg_log::*;
//!
//! fn main() {
//!     debug!("the answer is {}", 42);
//! }
//! ```
//!
//! is equivalent to
//!
//! ```rust
//! #[cfg(feature = "log")]
//! use log::*;
//!
//! fn main() {
//!     #[cfg(feature = "log")]
//!     debug!("the answer is {}", 42);
//! }

/// Equivalent to [`log::log_enabled!`](https://docs.rs/log/latest/log/macro.log_enabled.html)
/// if the feature `log` is enabled, `false` otherwise.
#[macro_export]
macro_rules! log_enabled {
    ($($tt:tt)*) => {{
        #[cfg(feature = "log")]
        {
            ::log::log_enabled!($($tt)*)
        }
        #[cfg(not(feature = "log"))]
        {
            false
        }
    }};
}

/// Equivalent to [`log::log!`](https://docs.rs/log/latest/log/macro.log.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::log!($($tt)*);
    };
}

/// Equivalent to [`log::trace!`](https://docs.rs/log/latest/log/macro.trace.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! trace {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::trace!($($tt)*);
    };
}

/// Equivalent to [`log::debug!`](https://docs.rs/log/latest/log/macro.debug.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! debug {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::debug!($($tt)*);
    };
}

/// Equivalent to [`log::info!`](https://docs.rs/log/latest/log/macro.info.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! info {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::info!($($tt)*);
    };
}

/// Equivalent to [`log::warn!`](https://docs.rs/log/latest/log/macro.warn.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::warn!($($tt)*);
    };
}

/// Equivalent to [`log::error!`](https://docs.rs/log/latest/log/macro.error.html)
/// if the feature `log` is enabled, discarded otherwise.
#[macro_export]
macro_rules! error {
    ($($tt:tt)*) => {
        #[cfg(feature="log")]
        ::log::error!($($tt)*);
    };
}
