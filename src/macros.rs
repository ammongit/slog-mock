/*
 * macros.rs
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

/// Dummy logging macro, mimics `slog::slog_o!`.
#[macro_export]
macro_rules! slog_o {
    ($($key:expr => $value:expr,)+) => {
        slog_o!($($key => $value),+)
    };
    ($($key:expr => $value:expr),*) => {{
        $(
            let _ = $key;
            let _ = $value;
        )*

        ()
    }};
}

/// Dummy logging macro, mimics `slog::o!`.
///
/// See `slog_o!`.
#[macro_export]
macro_rules! o {
    ($($key:expr => $value:expr,)+) => {
        slog_o!($($key => $value),+)
    };
    ($($key:expr => $value:expr),*) => {
        slog_o!($(key => $value),*)
    };
}
