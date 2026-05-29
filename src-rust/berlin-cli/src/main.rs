// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `berlin` — the FSBerlin command-line interface.
//!
//! Phase 1 scaffold: this card establishes the binary and argument parsing
//! (`--help`, `--version`). The verbs — `init`, `validate`, `query`,
//! `watch`, `render-waypoint` — each land in their own Phase 1 cards.

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
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Version) | None => {
            println!(
                "berlin {} (core {})",
                env!("CARGO_PKG_VERSION"),
                berlin_core::version()
            );
        }
    }
}
