/*
 * log.rs
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

/// Dummy `slog::Logger`-replacement struct.
#[derive(Debug, Copy, Clone)]
pub struct Logger;

impl Logger {
    /// Dummy `slog::Logger::new()` method.
    pub fn new(&self, _: ()) -> &Logger {
        self
    }
}
