// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `berlin-core` — the FSBerlin substrate core library.
//!
//! Phase 1 scaffold: this card establishes the crate, a shared error type,
//! and a `Result` alias. The filesystem walker, frontmatter parser, link
//! resolution, and SQLite mirror each land in their own Phase 1 cards.

pub mod frontmatter;
pub mod index;
pub mod links;
pub mod model;
pub mod query;
pub mod validate;
pub mod walk;

use thiserror::Error;

/// Errors produced by the substrate core.
///
/// Variants are added as the substrate grows; the scaffold seeds it with the
/// I/O case so downstream cards have a `?`-able error from the start.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error while reading or writing the project tree.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A YAML parse or schema-shape error in frontmatter (includes rejection
    /// of unknown/retired fields such as the retired universal `status:`).
    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// A SQLite error from the index mirror.
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// The file had no `---`-delimited frontmatter block at its head.
    #[error("no frontmatter block found")]
    MissingFrontmatter,

    /// A malformed query expression.
    #[error("query error: {0}")]
    Query(String),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The substrate-core version, so the CLI reports a single source of truth.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn io_error_converts_via_from() {
        let err: Error = std::io::Error::other("boom").into();
        assert!(matches!(err, Error::Io(_)));
        assert_eq!(err.to_string(), "i/o error: boom");
    }
}
