/*
 * slog_call.rs
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

/// Core of the crate, the `slog__unused!` macro.
///
/// Handles macro parsing and expansion of a slog-like
/// macro which consumes all its inputs as unused.

use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Token};
use std::fmt::{self, Debug};

pub struct SlogCall {
    pub logger: Expr,
    pub message: Expr,
    pub format_args: Vec<Expr>,
    pub context_keys: Vec<Expr>,
    pub context_values: Vec<Expr>,
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

impl Debug for SlogCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SlogCall")
    }
}
