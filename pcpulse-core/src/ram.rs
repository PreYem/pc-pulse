//! RAM and swap category probe.

use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

pub fn probe_ram_swap(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let ram_total = ctx.system.total_memory();

    if ram_total == 0 {
        add_missing(snapshot, StatId::Ram, MissReason::NoData);
    } else {
        snapshot.ram_total_b = Some(ram_total);
        snapshot.ram_used_b = Some(ctx.system.used_memory());
    }

    let swap_total = ctx.system.total_swap();

    if swap_total == 0 {
        add_missing(snapshot, StatId::Swap, MissReason::NoData);
    } else {
        snapshot.swap_total_b = Some(swap_total);
        snapshot.swap_used_b = Some(ctx.system.used_swap());
    }
}
