//! sysinfo snapshot collector.
//!
//! Builds a fresh `ProbeCtx`, then drives the HOT/COLD schedule once and returns a
//! single snapshot. Missing values are explicit `None` fields accompanied by an
//! entry in `missing` so the UI can display an honest reason instead of inventing
//! data. The live loop in `live.rs` reuses the same schedule against a persistent
//! context; the one-shot path here is what `--snapshot-once` and the GUI first
//! frame use.

use crate::config::{Config, default_config};
use crate::model::{MissReason, Snapshot};
use crate::registry::{all_missing, new_probe_ctx, run_cold, run_hot};

use sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
use std::thread::sleep;

pub fn collect_once(config: &Config) -> Snapshot {
    match new_probe_ctx() {
        None => {
            all_missing(MissReason::CollectorError)
        },
        Some(mut ctx) => {
            let mut snapshot = Snapshot::default();

            // CPU usage is diff-based: refresh once, wait at least the platform
            // interval, then refresh again so the first snapshot is already accurate.
            sleep(MINIMUM_CPU_UPDATE_INTERVAL);

            run_cold(&mut snapshot, &mut ctx, config);
            run_hot(&mut snapshot, &mut ctx, config);

            snapshot
        },
    }
}

pub fn collect_snapshot_once() -> Snapshot {
    collect_once(&default_config())
}
