test_cfg_log
============

This is an example package which demonstrates how to use the [`cfg_log`] crate.

This crate depends on [`cfg_log`] and optionally on [`log`].
All logging is done through the [`cfg_log::*` macros][cfg_log_macros],
which call the [`log::*` macros][log_macros] when the `log` feature
is enabled and are discarded otherwise.

  [`cfg_log`]: https://crates.io/crates/cfg_log
  [`log`]: https://crates.io/crates/log
  [cfg_log_macros]: https://docs.rs/cfg_log/latest/cfg_log/#macros
  [log_macros]: https://docs.rs/log/latest/log/#macros

How to run
----------

- Run with the `log` feature disabled (default):

  ```bash
  ❯ cargo run
      Finished dev [unoptimized + debuginfo] target(s) in 0.01s
       Running `/home/fstra/Code/rust/cfg_log/target/debug/test_cfg_log`
  Debug disabled
  ```

- Run with the `log` feature enabled:

  ```bash
  ❯ cargo run -F log
      Finished dev [unoptimized + debuginfo] target(s) in 0.01s
       Running `/home/fstra/Code/rust/cfg_log/target/debug/test_cfg_log`
  Debug enabled
  2022-08-29T19:31:02.812+02:00 INFO [test_cfg_log] answer = 42
  2022-08-29T19:31:02.812+02:00 TRACE [test_cfg_log] the answer is 42
  2022-08-29T19:31:02.812+02:00 DEBUG [test_cfg_log] the answer is 42
  2022-08-29T19:31:02.812+02:00 INFO [test_cfg_log] the answer is 42
  2022-08-29T19:31:02.812+02:00 WARN [test_cfg_log] the answer is 42
  2022-08-29T19:31:02.812+02:00 ERROR [test_cfg_log] the answer is 42
  2022-08-29T19:31:02.812+02:00 INFO [tgt] answer = 42
  2022-08-29T19:31:02.812+02:00 TRACE [tgt] the answer is 42
  2022-08-29T19:31:02.812+02:00 DEBUG [tgt] the answer is 42
  2022-08-29T19:31:02.812+02:00 INFO [tgt] the answer is 42
  2022-08-29T19:31:02.813+02:00 WARN [tgt] the answer is 42
  2022-08-29T19:31:02.813+02:00 ERROR [tgt] the answer is 42
  ```

- Run with the `log` feature enabled and log level set to `warn`:

  ```bash
  ❯ RUST_LOG=warn cargo run -F log
      Finished dev [unoptimized + debuginfo] target(s) in 0.02s
       Running `/home/fstra/Code/rust/cfg_log/target/debug/test_cfg_log`
  Debug disabled
  2022-08-29T19:31:08.426+02:00 WARN [test_cfg_log] the answer is 42
  2022-08-29T19:31:08.426+02:00 ERROR [test_cfg_log] the answer is 42
  2022-08-29T19:31:08.426+02:00 WARN [tgt] the answer is 42
  2022-08-29T19:31:08.426+02:00 ERROR [tgt] the answer is 42
  ```
