//! Disk usage category probe.

use crate::model::{DiskStat, MissReason, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

pub fn probe_disks(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    snapshot.disks.clear();

    for disk in &ctx.disks {
        let total = disk.total_space();
        let free = disk.available_space();

        if total > 0 {
            snapshot.disks.push(DiskStat {
                mount: disk.mount_point().display().to_string(),
                total_b: total,
                free_b: free,
            });
        }
    }

    if snapshot.disks.is_empty() {
        add_missing(snapshot, StatId::DiskUsage, MissReason::NoData);
    }
}
