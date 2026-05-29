// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `berlin init`: scaffold a new, valid FSBerlin project on disk (SPEC §2.1).

use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ulid::Ulid;

use crate::{Error, Result};

/// Create a new FSBerlin project at `root`.
///
/// Refuses to write into a non-empty directory (no clobber). Creates the
/// standard layout, a generated project ULID, today's `created` date, and a
/// stub `why.md`. The result validates clean.
pub fn init(root: &Path) -> Result<()> {
    if root.exists() {
        if root.is_file() {
            return Err(Error::Init(format!("{} is a file", root.display())));
        }
        if fs::read_dir(root)?.next().is_some() {
            return Err(Error::Init(format!(
                "{} is not empty (refusing to clobber)",
                root.display()
            )));
        }
    } else {
        fs::create_dir_all(root)?;
    }

    for dir in [
        "cards",
        "agents",
        "waypoints",
        "findings/spymaster",
        "findings/sentinel",
        ".fsberlin",
    ] {
        fs::create_dir_all(root.join(dir))?;
    }

    let name = root.file_name().map_or_else(
        || "project".to_string(),
        |n| n.to_string_lossy().into_owned(),
    );

    let config = format!(
        "project_id: {}\nname: {}\ncreated: {}\nschema_version: \"0.1\"\n",
        Ulid::new(),
        name,
        today_iso(),
    );
    fs::write(root.join(".fsberlin").join("config.yaml"), config)?;

    let why = format!("# Why {name}\n\n(Describe why this project exists, in plain language.)\n");
    fs::write(root.join("why.md"), why)?;

    Ok(())
}

/// Today's date as `YYYY-MM-DD` (UTC), via Howard Hinnant's civil-from-days
/// algorithm — so this single use doesn't pull in a calendar crate.
fn today_iso() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = i64::try_from(secs / 86_400).unwrap_or(0);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::validate;
    use tempfile::tempdir;

    #[test]
    fn init_creates_a_project_that_validates() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("my-project");
        init(&root).unwrap();

        assert!(root.join(".fsberlin/config.yaml").is_file());
        assert!(root.join("why.md").is_file());
        assert!(root.join("cards").is_dir());
        assert!(root.join("findings/spymaster").is_dir());

        let findings = validate(&root).unwrap();
        assert!(
            findings.is_empty(),
            "a freshly initialized project must validate clean: {findings:?}"
        );
    }

    #[test]
    fn init_refuses_a_non_empty_target() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("existing.txt"), "x").unwrap();
        let err = init(dir.path()).unwrap_err();
        assert!(matches!(err, Error::Init(_)));
    }

    #[test]
    fn rerun_refuses_no_clobber() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("p");
        init(&root).unwrap();
        // Second run sees a populated project and refuses.
        assert!(matches!(init(&root).unwrap_err(), Error::Init(_)));
    }

    #[test]
    fn today_iso_is_well_formed() {
        let d = today_iso();
        assert_eq!(d.len(), 10);
        assert_eq!(d.matches('-').count(), 2);
    }
}
