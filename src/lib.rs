/*
 * lib.rs
 *
 * slog-mock - Mock crate for slog to compile out all logging.
 * Copyright (c) 2021 Ammon Smith
 *
 * slog-mock is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

//! Crate to mock [`slog`] objects and macros.
//!
//! This exports many expected pieces of functionality
//! from the `slog` crate, but has them compile into no-ops.
//!
//! The goal is to allow a crate consumer to have a feature where
//! this crate is used instead of `slog`, and all logging calls
//! resolve cleanly but don't produce any code in the final binary.
//!
//! Not all of `slog`'s functionality is mocked: you will need to
//! gate any actual requirements on its traits or structures
//! behind `#[cfg(feature)]` checks.
//!
//! [`slog`]: https://crates.io/crates/slog

#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]

extern crate slog_mock_proc_macros;

mod log;
mod macros;

pub use self::log::Logger;
pub use slog_mock_proc_macros::*;
