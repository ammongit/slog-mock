## slog-mock

<p>
  <a href="https://github.com/ammongit/slog-mock/actions?query=workflow%3A%22Rust%22">
    <img src="https://github.com/ammongit/slog-mock/workflows/Rust/badge.svg"
         alt="Build status">
  </a>
</p>

Crate to mock [`slog`](https://crates.io/crates/slog), returning macros which mimic slog's.

This is not a full drop-in replacement for `slog`, but a limited subset of its functionality.
The goal is to enable crates to provide a feature that compiles out all logging code entirely,
in cases where logging is not needed for a specific target due to sensitive performance concerns.

The lint `#![forbid(unsafe_code)]` is set, and therefore this crate has only safe code.

Available under the terms of the MIT License.

### Compilation

This library targets the latest stable Rust. At time of writing, that is `1.55.0`.

```sh
$ cargo build --release
```
