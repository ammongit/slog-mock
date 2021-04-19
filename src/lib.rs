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

#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]

//! Crate to mock [`slog`] macros.
//!
//! This exports many expected macros from the `slog` crate,
//! but has them compile into no-ops.
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

extern crate proc_macro;
extern crate syn;
extern crate quote;

#[cfg(test)]
mod test;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr};

struct SlogCall {
    logger: Expr,
}

impl Parse for SlogCall {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}

#[proc_macro]
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn slog__unused(input: TokenStream) -> TokenStream {
    let SlogCall { .. } = parse_macro_input!(input as SlogCall);
    todo!();
}
