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

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

#[cfg(test)]
mod test;

mod slog_call;

use proc_macro::TokenStream;
use self::slog_call::SlogCall;

#[proc_macro]
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn slog__unused(input: TokenStream) -> TokenStream {
    let SlogCall {
        logger,
        message,
        format_args,
        context_keys,
        context_values,
    } = parse_macro_input!(input as SlogCall);

    let expanded = quote! {{
        let _ = #logger;
        let _ = #message;

        #(
            let _ = #format_args;
        )*

        #(
            let _ = #context_keys;
        )*

        #(
            let  = #context_values;
        )*

        ()
    }};

    TokenStream::from(expanded)
}

/// Dummy logging macro, mimics `slog::crit!`.
#[proc_macro]
pub fn crit(input: TokenStream) -> TokenStream {
    slog__unused(input)
}

/// Dummy logging macro, mimics `slog::error!`.
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    slog__unused(input)
}

/// Dummy logging macro, mimics `slog::warn!`.
#[proc_macro]
pub fn warn(input: TokenStream) -> TokenStream {
    slog__unused(input)
}

/// Dummy logging macro, mimics `slog::info!`.
#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    slog__unused(input)
}

/// Dummy logging macro, mimics `slog::debug!`.
#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    slog__unused(input)
}

/// Dummy logging macro, mimics `slog::trace!`.
#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    slog__unused(input)
}
