// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `berlin` — the FSBerlin command-line interface.
//!
//! Phase 1 scaffold: this card establishes the binary and argument parsing
//! (`--help`, `--version`). The verbs — `init`, `validate`, `query`,
//! `watch`, `render-waypoint` — each land in their own Phase 1 cards.

use std::path::PathBuf;
use std::process::ExitCode;

use berlin_core::{index, query};
use clap::{Parser, Subcommand};

/// FSBerlin — a file-hierarchy project-management substrate.
#[derive(Debug, Parser)]
#[command(name = "berlin", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the CLI and substrate-core versions.
    Version,
    /// Query the card index, e.g. "type:card AND building_status:in-progress".
    Query {
        /// The query expression: `field:value` predicates joined by AND/OR.
        expr: String,
        /// Project root to query.
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> berlin_core::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        None | Some(Command::Version) => {
            println!(
                "berlin {} (core {})",
                env!("CARGO_PKG_VERSION"),
                berlin_core::version()
            );
        }
        Some(Command::Query { expr, path }) => {
            let conn = index::build_in_memory(&path)?;
            let matches = query::run(&conn, &expr)?;
            if matches.is_empty() {
                println!("(no matches)");
            } else {
                for (slug, title) in matches {
                    println!("{slug}  {title}");
                }
            }
        }
    }
    Ok(())
}
