/*
 * slog_call.rs
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

use std::fmt::{self, Debug};
/// Core of the crate, the `slog__unused!` macro.
///
/// Handles macro parsing and expansion of a slog-like
/// macro which consumes all its inputs as unused.
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, Token};

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

        macro_rules! get_optional {
            ($token:tt) => {
                if input.peek(Token![$token]) {
                    input.parse::<Token![$token]>()?;
                }
            };
        }

        // Get message formatting
        get_optional![,];
        check_done!();

        while !input.peek(Token![;]) {
            let format_arg: Expr = input.parse()?;

            format_args.push(format_arg);
            get_optional![,];
            check_done!();
        }

        // Get key-value context arguments
        input.parse::<Token![;]>()?;
        check_done!();

        loop {
            let key: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let value: Expr = input.parse()?;

            context_keys.push(key);
            context_values.push(value);

            get_optional![,];
            check_done!();
        }
    }
}

impl Debug for SlogCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SlogCall")
    }
}
