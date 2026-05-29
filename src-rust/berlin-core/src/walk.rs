// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Discovering the FSBerlin project layout on disk (SPEC §2.1) without ever
//! descending into tool-owned directories (ADR-008).
//!
//! Discovery is *targeted*: it reads only `cards/`, `waypoints/`, `agents/`,
//! and `.fsberlin/`, so opaque directories like `.git` and `.beads` are never
//! read. [`is_opaque`] is exposed for the future full-tree validator and
//! watcher, and is also applied defensively here.

use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;

/// Tool-owned directories the substrate must not scan — the ADR-008 defaults,
/// mirroring `schema/project.schema.yaml`.
pub const DEFAULT_OPAQUE_PATHS: &[&str] = &[
    ".git",
    ".beads",
    ".github",
    "node_modules",
    "target",
    "__pycache__",
    ".idea",
    ".vscode",
];

/// Whether a directory name is opaque by default (ADR-008).
#[must_use]
pub fn is_opaque(name: &str) -> bool {
    DEFAULT_OPAQUE_PATHS.contains(&name)
}

/// A discovered card folder (`cards/<slug>/card.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardEntry {
    pub slug: String,
    pub path: PathBuf,
}

/// A discovered waypoint folder (`waypoints/<slug>/waypoint.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaypointEntry {
    pub slug: String,
    pub path: PathBuf,
}

/// A discovered agent definition (`agents/<id>.yaml`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEntry {
    pub id: String,
    pub path: PathBuf,
}

/// The discovered shape of a project.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Layout {
    pub root: PathBuf,
    pub cards: Vec<CardEntry>,
    pub waypoints: Vec<WaypointEntry>,
    pub agents: Vec<AgentEntry>,
    pub config: Option<PathBuf>,
    pub findings_dir: Option<PathBuf>,
}

/// Discover the layout of the project rooted at `root`.
///
/// Reads only the known substrate directories; never descends into opaque
/// directories (ADR-008). Missing directories yield empty collections rather
/// than errors.
pub fn discover(root: &Path) -> Result<Layout> {
    let cards = folders_with_file(&root.join("cards"), "card.md")?
        .into_iter()
        .map(|(slug, path)| CardEntry { slug, path })
        .collect();
    let waypoints = folders_with_file(&root.join("waypoints"), "waypoint.md")?
        .into_iter()
        .map(|(slug, path)| WaypointEntry { slug, path })
        .collect();
    let agents = yaml_files(&root.join("agents"))?
        .into_iter()
        .map(|(id, path)| AgentEntry { id, path })
        .collect();

    let cfg = root.join(".fsberlin").join("config.yaml");
    let config = cfg.is_file().then_some(cfg);
    let findings = root.join("findings");
    let findings_dir = findings.is_dir().then_some(findings);

    Ok(Layout {
        root: root.to_path_buf(),
        cards,
        waypoints,
        agents,
        config,
        findings_dir,
    })
}

/// Subdirectories of `dir` containing `file`, as `(slug, path-to-file)`,
/// sorted by slug. Opaque-named subdirectories are skipped (ADR-008).
fn folders_with_file(dir: &Path, file: &str) -> Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(out); // a missing directory is simply empty
    };
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_opaque(&name) {
            continue;
        }
        let candidate = entry.path().join(file);
        if candidate.is_file() {
            out.push((name, candidate));
        }
    }
    out.sort();
    Ok(out)
}

/// `*.yaml` files directly in `dir`, as `(stem, path)`, sorted by stem.
fn yaml_files(dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(out);
    };
    for entry in entries {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "yaml")
            && let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().into_owned())
        {
            out.push((stem, path));
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn touch(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "x").unwrap();
    }

    #[test]
    fn is_opaque_knows_tool_dirs() {
        assert!(is_opaque(".beads"));
        assert!(is_opaque(".git"));
        assert!(!is_opaque("cards"));
    }

    #[test]
    fn discover_finds_substrate_and_skips_opaque() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        touch(&root.join("cards/real-card/card.md"));
        touch(&root.join("cards/.beads/card.md")); // opaque-named folder under cards/
        touch(&root.join("waypoints/wp-1/waypoint.md"));
        touch(&root.join("agents/lari.yaml"));
        touch(&root.join(".fsberlin/config.yaml"));
        touch(&root.join(".beads/some.db")); // opaque sibling, must never be read
        fs::create_dir_all(root.join(".git")).unwrap();

        let layout = discover(root).unwrap();

        assert_eq!(layout.cards.len(), 1);
        assert_eq!(layout.cards[0].slug, "real-card");
        assert_eq!(layout.waypoints.len(), 1);
        assert_eq!(layout.agents.len(), 1);
        assert_eq!(layout.agents[0].id, "lari");
        assert!(layout.config.is_some());

        let discovered = layout
            .cards
            .iter()
            .map(|c| c.path.clone())
            .chain(layout.agents.iter().map(|a| a.path.clone()))
            .chain(layout.waypoints.iter().map(|w| w.path.clone()));
        for p in discovered {
            let s = p.to_string_lossy();
            assert!(
                !s.contains("/.beads/") && !s.contains("/.git/"),
                "leaked an opaque path: {s}"
            );
        }
    }

    #[test]
    fn discover_missing_dirs_is_empty_not_error() {
        let dir = tempdir().unwrap();
        let layout = discover(dir.path()).unwrap();
        assert!(layout.cards.is_empty());
        assert!(layout.agents.is_empty());
        assert!(layout.config.is_none());
    }
}
