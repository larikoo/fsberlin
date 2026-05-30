// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `berlin` — the FSBerlin command-line interface.
//!
//! Phase 1 scaffold: this card establishes the binary and argument parsing
//! (`--help`, `--version`). The verbs — `init`, `validate`, `query`,
//! `watch`, `render-waypoint` — each land in their own Phase 1 cards.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use berlin_core::{index, init, query, render, validate, watch};
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
    /// Validate a project: YAML safe-load, schema shape, and references.
    Validate {
        /// Project root to validate.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Create a new FSBerlin project on disk.
    Init {
        /// Where to create the project.
        path: PathBuf,
    },
    /// Render a waypoint's projected state (base + overlay) into a directory.
    RenderWaypoint {
        /// The waypoint slug to render.
        slug: String,
        /// Project root.
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Output directory (must be empty/new and outside the project).
        #[arg(long)]
        out: PathBuf,
    },
    /// Watch a project and keep its index live (Ctrl-C to stop).
    Watch {
        /// Project root to watch.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Error early if `path` does not look like an FSBerlin project, so verbs
/// like `query` and `watch` give a clear message instead of silent empty
/// output. `validate` and `init` handle this themselves; `render-waypoint`
/// fails naturally on a missing waypoints dir.
fn require_project(path: &std::path::Path) -> berlin_core::Result<()> {
    let cfg = path.join(".fsberlin").join("config.yaml");
    if !cfg.is_file() {
        return Err(berlin_core::Error::Query(format!(
            "{} is not an FSBerlin project (no .fsberlin/config.yaml)",
            path.display()
        )));
    }
    Ok(())
}

fn run() -> berlin_core::Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        None | Some(Command::Version) => {
            println!(
                "berlin {} (core {})",
                env!("CARGO_PKG_VERSION"),
                berlin_core::version()
            );
            Ok(ExitCode::SUCCESS)
        }
        Some(Command::Query { expr, path }) => {
            require_project(&path)?;
            let conn = index::build_in_memory(&path)?;
            let matches = query::run(&conn, &expr)?;
            if matches.is_empty() {
                println!("(no matches)");
            } else {
                for (slug, title) in matches {
                    println!("{slug}  {title}");
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(Command::Validate { path }) => {
            let findings = validate::validate(&path)?;
            if findings.is_empty() {
                println!("ok: no problems");
                Ok(ExitCode::SUCCESS)
            } else {
                for finding in &findings {
                    eprintln!("{finding}");
                }
                eprintln!("{} problem(s)", findings.len());
                Ok(ExitCode::FAILURE)
            }
        }
        Some(Command::Init { path }) => {
            init::init(&path)?;
            println!("initialized FSBerlin project at {}", path.display());
            Ok(ExitCode::SUCCESS)
        }
        Some(Command::RenderWaypoint { slug, path, out }) => {
            let report = render::render(&path, &slug, &out)?;
            println!(
                "rendered waypoint `{slug}` -> {} ({} base files, {} overlaid)",
                out.display(),
                report.base_files,
                report.overlaid.len()
            );
            for rel in &report.overlaid {
                println!("  overlay: {rel}");
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(Command::Watch { path }) => {
            require_project(&path)?;
            let db = path.join(".fsberlin").join("index.sqlite");
            println!("watching {} (Ctrl-C to stop)", path.display());
            let stop = AtomicBool::new(false);
            watch::watch(&path, &db, Duration::from_millis(200), &stop)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
