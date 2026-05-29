// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Rendering a waypoint's projected state: `projected = base + overlay`
//! (ADR-003). The base is the project tree (minus `waypoints/`, opaque
//! directories, and the regenerable index cache); the overlay is the files
//! under `waypoints/<slug>/` (excluding its `waypoint.md` descriptor), which
//! shadow same-relative-path base files. Invariant-floor files (root
//! `why.md`, `schema/*`) may not be overlaid.

use std::fs;
use std::path::Path;

use crate::walk::is_opaque;
use crate::{Error, Result};

/// What a render produced.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RenderReport {
    /// Number of base files copied.
    pub base_files: usize,
    /// Relative paths contributed by the overlay (added or shadowed), sorted.
    pub overlaid: Vec<String>,
}

/// Render waypoint `slug` of the project at `root` into the `out` directory.
///
/// `out` must be empty or not yet exist (no clobber) and must lie outside the
/// project root.
pub fn render(root: &Path, slug: &str, out: &Path) -> Result<RenderReport> {
    let waypoint_dir = root.join("waypoints").join(slug);
    if !waypoint_dir.is_dir() {
        return Err(Error::Render(format!("no waypoint `{slug}`")));
    }
    if out.starts_with(root) {
        return Err(Error::Render(
            "output directory must be outside the project".into(),
        ));
    }
    if out.is_dir() && fs::read_dir(out)?.next().is_some() {
        return Err(Error::Render(format!(
            "{} is not empty (refusing to clobber)",
            out.display()
        )));
    }
    fs::create_dir_all(out)?;

    let mut base_files = 0;
    copy_base(root, root, out, &mut base_files)?;

    let mut overlaid = Vec::new();
    apply_overlay(&waypoint_dir, &waypoint_dir, out, &mut overlaid)?;
    overlaid.sort();

    Ok(RenderReport {
        base_files,
        overlaid,
    })
}

/// Recursively copy base files into `out`, skipping `waypoints/` (the overlays
/// themselves), opaque directories, and the regenerable index cache.
fn copy_base(root: &Path, dir: &Path, out: &Path, count: &mut usize) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if file_type.is_dir() {
            if is_opaque(&name) || (dir == root && name == "waypoints") {
                continue;
            }
            copy_base(root, &path, out, count)?;
        } else if file_type.is_file() {
            let rel = path.strip_prefix(root).unwrap_or(&path);
            if rel == Path::new(".fsberlin/index.sqlite") {
                continue;
            }
            write_into(out, rel, &path)?;
            *count += 1;
        }
    }
    Ok(())
}

/// Recursively apply overlay files (rooted at `waypoint_dir`) into `out`,
/// rejecting any attempt to shadow an invariant-floor file.
fn apply_overlay(
    waypoint_dir: &Path,
    dir: &Path,
    out: &Path,
    overlaid: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            apply_overlay(waypoint_dir, &path, out, overlaid)?;
        } else if file_type.is_file() {
            let rel = path.strip_prefix(waypoint_dir).unwrap_or(&path);
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            // The waypoint descriptor is metadata, not an overlay file.
            if rel_str == "waypoint.md" {
                continue;
            }
            if is_invariant_floor(&rel_str) {
                return Err(Error::Render(format!(
                    "overlay may not shadow invariant-floor file `{rel_str}`"
                )));
            }
            write_into(out, rel, &path)?;
            overlaid.push(rel_str);
        }
    }
    Ok(())
}

fn write_into(out: &Path, rel: &Path, src: &Path) -> Result<()> {
    let dest = out.join(rel);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, &dest)?;
    Ok(())
}

fn is_invariant_floor(rel: &str) -> bool {
    rel == "why.md" || rel == "schema" || rel.starts_with("schema/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn project(root: &Path) {
        write(&root.join("why.md"), "# base why\n");
        write(&root.join("cards/c/card.md"), "base card\n");
        write(&root.join(".fsberlin/config.yaml"), "name: t\n");
    }

    #[test]
    fn renders_base_plus_overlay() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("proj");
        project(&root);
        // overlay: shadow the card and add a new file; plus a descriptor.
        write(
            &root.join("waypoints/wp/waypoint.md"),
            "---\ntype: waypoint\n---\n",
        );
        write(
            &root.join("waypoints/wp/cards/c/card.md"),
            "overlaid card\n",
        );
        write(&root.join("waypoints/wp/extra.md"), "new at milestone\n");

        let out = tempdir().unwrap();
        let report = render(&root, "wp", &out.path().join("rendered")).unwrap();

        let outp = out.path().join("rendered");
        assert_eq!(
            fs::read_to_string(outp.join("why.md")).unwrap(),
            "# base why\n"
        );
        // overlay shadowed the base card
        assert_eq!(
            fs::read_to_string(outp.join("cards/c/card.md")).unwrap(),
            "overlaid card\n"
        );
        assert_eq!(
            fs::read_to_string(outp.join("extra.md")).unwrap(),
            "new at milestone\n"
        );
        // descriptor is not projected
        assert!(!outp.join("waypoint.md").exists());
        assert_eq!(report.overlaid, ["cards/c/card.md", "extra.md"]);
        assert!(report.base_files >= 3);
    }

    #[test]
    fn rejects_invariant_floor_overlay() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("proj");
        project(&root);
        write(&root.join("waypoints/wp/why.md"), "hijacked\n");
        let out = tempdir().unwrap();
        let err = render(&root, "wp", &out.path().join("r")).unwrap_err();
        assert!(matches!(err, Error::Render(_)));
        assert!(err.to_string().contains("why.md"));
    }

    #[test]
    fn unknown_waypoint_errors() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("proj");
        project(&root);
        let out = tempdir().unwrap();
        assert!(matches!(
            render(&root, "nope", &out.path().join("r")),
            Err(Error::Render(_))
        ));
    }
}
