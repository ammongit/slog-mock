## slog-mock

Crate to mock [`slog`](https://crates.io/crates/slog), returning macros which mimic slog's.

This is not a full drop-in replacement for `slog`, but a limited subset of its functionality.
The goal is to enable crates to provide a feature that compiles out all logging code entirely,
in cases where logging is not needed for a specific target due to sensitive performance concerns.

Available under the terms of the MIT License.
