// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Live (timing- and OS-dependent) watcher test. `#[ignore]` by default — run
//! with `cargo test -- --ignored` — so the normal suite stays fast and
//! flake-free while this still exists to exercise the real notify path.

use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use berlin_core::{index, query, watch};

fn card(root: &Path, slug: &str, status: &str) {
    let p = root.join("cards").join(slug).join("card.md");
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(
        &p,
        format!(
            "---\nid: 01{slug}\ntitle: {slug}\ntype: card\nbuilding_status: {status}\ncreated: 2026-05-29\n---\n"
        ),
    )
    .unwrap();
}

#[test]
#[ignore = "live filesystem watcher; run with --ignored"]
fn watcher_picks_up_a_new_card() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    fs::create_dir_all(root.join(".fsberlin")).unwrap();
    card(&root, "a", "pending");

    let db = root.join(".fsberlin").join("index.sqlite");
    let stop = Arc::new(AtomicBool::new(false));

    let r = root.clone();
    let d = db.clone();
    let s = Arc::clone(&stop);
    let handle = thread::spawn(move || {
        let _ = watch::watch(&r, &d, Duration::from_millis(100), &s);
    });

    thread::sleep(Duration::from_millis(600)); // let the initial build settle
    card(&root, "b", "done"); // add a second card

    let mut ok = false;
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(100));
        if let Ok(conn) = index::open(&db)
            && let Ok(rows) = query::run(&conn, "type:card")
            && rows.len() == 2
        {
            ok = true;
            break;
        }
    }

    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    assert!(
        ok,
        "watcher did not reflect the new card within the timeout"
    );
}
