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

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Token};

struct SlogCall {
    logger: Expr,
    message: Expr,
    format_args: Vec<Expr>,
    context_keys: Vec<Expr>,
    context_values: Vec<Expr>,
}

impl Parse for SlogCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut format_args = Vec::new();
        let mut context_keys = Vec::new();
        let mut context_values = Vec::new();

        let logger: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let message: Expr = input.parse()?;

        macro_rules! check_done {
            () => {
                if input.is_empty() {
                    return Ok(SlogCall {
                        logger,
                        message,
                        format_args,
                        context_keys,
                        context_values,
                    });
                }
            };
        }

        // Get message formatting
        check_done!();

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let format_arg: Expr = input.parse()?;

            format_args.push(format_arg);
        }

        // Get key-value context arguments
        check_done!();
        input.parse::<Token![;]>()?;

        loop {
            let key: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let value: Expr = input.parse()?;

            context_keys.push(key);
            context_values.push(value);

            check_done!();
            input.parse::<Token![,]>()?;
            check_done!();
        }
    }
}

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
