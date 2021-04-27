/*
 * lib.rs
 *
 * slog-mock-proc-macros - Mock crate for slog to compile out all logging.
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

//! Inner crate to implement procedural macros for `slog-mock`.

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod slog_call;

use self::slog_call::SlogCall;
use proc_macro::TokenStream;

fn slog_unused(input: TokenStream) -> TokenStream {
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
            let _ = #context_values;
        )*

        ()
    }};

    TokenStream::from(expanded)
}

/// Dummy logging macro, mimics `slog::crit!`.
#[proc_macro]
pub fn crit(input: TokenStream) -> TokenStream {
    slog_unused(input)
}

/// Dummy logging macro, mimics `slog::error!`.
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    slog_unused(input)
}

/// Dummy logging macro, mimics `slog::warn!`.
#[proc_macro]
pub fn warn(input: TokenStream) -> TokenStream {
    slog_unused(input)
}

/// Dummy logging macro, mimics `slog::info!`.
#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    slog_unused(input)
}

/// Dummy logging macro, mimics `slog::debug!`.
#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    slog_unused(input)
}

/// Dummy logging macro, mimics `slog::trace!`.
#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    slog_unused(input)
}
