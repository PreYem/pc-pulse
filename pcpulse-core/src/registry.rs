//! Central probe registry: which per-stat probes run, and in which cost group.
//!
//! Every stat is collected by a single clearly-named probe function that lives
//! in its category file (`cpu.rs`, `ram.rs`, `disk.rs`, `gpu.rs`, `lhm.rs`, ...).
//! This file owns the shared `ProbeCtx` (the long-lived sysinfo state we reuse
//! across ticks so we do not re-enumerate every OS list each time) and the two
//! schedule functions `run_hot` and `run_cold` that list which probes fire per
//! tick. The live collector (`live.rs`) and the one-shot collector (`collector.rs`)
//! both drive the same schedule so the TUI and the future GUI read one snapshot.
//!
//! Cost groups:
//! - HOT  (every 500 ms): cpu usage/per-core/freq, cpu temp, ram, swap,
//!   GPU utilization, GPU temp/fan/power/SMART (Windows LHM).
//! - COLD (every ~30 s):  uptime, disks, network counters, top processes, GPU identity.
//!
//! Values are never faked: every probe leaves a field `None` and records an
//! entry in `missing` (see `add_missing`) so the UI can render an honest reason.

use crate::config::Config;
use crate::model::{MissReason, Snapshot, StatId};

use sysinfo::{
    Components,
    Disks,
    Networks,
    RefreshKind,
    System,
};

#[cfg(not(windows))]
use sysinfo::Gpus;

pub struct ProbeCtx {
    pub system: System,
    pub components: Components,
    pub disks: Disks,
    pub networks: Networks,
    #[cfg(not(windows))]
    pub gpus: Gpus,
}

pub fn add_missing(snapshot: &mut Snapshot, id: StatId, reason: MissReason) {
    for entry in &mut snapshot.missing {
        if entry.0 == id {
            entry.1 = reason;
            return;
        }
    }

    snapshot.missing.push((id, reason));
}

pub fn basic_ids() -> [StatId; 9] {
    [
        StatId::Uptime,
        StatId::CpuUsage,
        StatId::CpuFreq,
        StatId::CpuTemp,
        StatId::Ram,
        StatId::Swap,
        StatId::DiskUsage,
        StatId::NetIo,
        StatId::TopProcesses,
    ]
}

pub fn not_implemented_ids() -> [StatId; 6] {
    [
        StatId::GpuTemp,
        StatId::GpuUtil,
        StatId::GpuVram,
        StatId::FanSpeed,
        StatId::PowerDraw,
        StatId::SsdSmart,
    ]
}

pub fn lhm_ids() -> [StatId; 5] {
    [
        StatId::GpuTemp,
        StatId::GpuUtil,
        StatId::PowerDraw,
        StatId::FanSpeed,
        StatId::SsdSmart,
    ]
}

pub fn all_missing(reason: MissReason) -> Snapshot {
    let mut snapshot = Snapshot::default();

    for id in basic_ids() {
        add_missing(&mut snapshot, id, reason.clone());
    }

    for id in not_implemented_ids() {
        add_missing(&mut snapshot, id, MissReason::CollectorNotImplemented);
    }

    snapshot
}

pub fn new_probe_ctx() -> Option<ProbeCtx> {
    let kind = RefreshKind::nothing()
        .with_cpu(cpu_refresh())
        .with_memory(memory_refresh())
        .with_processes(proc_refresh());

    let system = match System::new_with_specifics(kind) {
        Ok(system) => system,
        Err(_) => return None,
    };

    let components = match Components::new_with_refreshed_list() {
        Ok(components) => components,
        Err(_) => return None,
    };

    let disks = match Disks::new_with_refreshed_list() {
        Ok(disks) => disks,
        Err(_) => return None,
    };

    let networks = match Networks::new_with_refreshed_list() {
        Ok(networks) => networks,
        Err(_) => return None,
    };

    #[cfg(not(windows))]
    let gpus = match Gpus::new_with_refreshed_list() {
        Ok(gpus) => gpus,
        Err(_) => return None,
    };

    Some(ProbeCtx {
        system,
        components,
        disks,
        networks,
        #[cfg(not(windows))]
        gpus,
    })
}

pub fn run_hot(snapshot: &mut Snapshot, ctx: &mut ProbeCtx, config: &Config) {
    ctx.system.refresh_cpu_usage();
    ctx.system.refresh_memory();
    ctx.components.refresh(false);
    #[cfg(not(windows))]
    ctx.gpus.refresh(true);

    crate::cpu::probe_cpu(snapshot, ctx, config);
    crate::cpu::probe_cpu_temp(snapshot, ctx);
    crate::ram::probe_ram_swap(snapshot, ctx);

    #[cfg(windows)]
    crate::lhm::probe_lhm(snapshot, config);
    #[cfg(not(windows))]
    crate::gpu::probe_gpu_util(snapshot, ctx);
}

pub fn run_cold(snapshot: &mut Snapshot, ctx: &mut ProbeCtx, config: &Config) {
    ctx.disks.refresh(false);
    ctx.networks.refresh(false);

    crate::uptime::probe_uptime(snapshot);
    crate::disk::probe_disks(snapshot, ctx);
    crate::net::probe_net(snapshot, ctx);
    crate::procs::probe_procs(snapshot, ctx, config);
    crate::gpu::probe_identity(snapshot, ctx);
}

fn cpu_refresh() -> sysinfo::CpuRefreshKind {
    sysinfo::CpuRefreshKind::nothing().with_usage().with_frequency()
}

fn memory_refresh() -> sysinfo::MemoryRefreshKind {
    sysinfo::MemoryRefreshKind::nothing().with_ram().with_swap()
}

fn proc_refresh() -> sysinfo::ProcessRefreshKind {
    sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory()
}
