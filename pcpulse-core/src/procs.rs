//! Top-process category probe.

use std::cmp::Ordering;

use crate::config::Config;
use crate::model::{MissReason, ProcStat, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

fn cmp_cpu_desc(a: &ProcStat, b: &ProcStat) -> Ordering {
    b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(Ordering::Equal)
}

fn cmp_mem_desc(a: &ProcStat, b: &ProcStat) -> Ordering {
    b.mem_b.cmp(&a.mem_b)
}

pub fn probe_procs(snapshot: &mut Snapshot, ctx: &ProbeCtx, config: &Config) {
    let mut top_cpu: Vec<ProcStat> = Vec::new();
    let mut top_mem: Vec<ProcStat> = Vec::new();

    for proc in ctx.system.processes().values() {
        let name = proc.name().to_string_lossy().to_string();
        let pid = proc.pid().as_u32();
        let mem_b = proc.memory();
        let cpu_pct = proc.cpu_usage() as f64;

        if cpu_pct == 0.0 && mem_b == 0 {
            continue;
        }

        top_cpu.push(ProcStat {
            name: name.clone(),
            pid,
            cpu_pct,
            mem_b,
        });

        top_mem.push(ProcStat {
            name,
            pid,
            cpu_pct,
            mem_b,
        });
    }

    top_cpu.sort_by(cmp_cpu_desc);
    top_mem.sort_by(cmp_mem_desc);

    let top_n = if config.top_processes == 0_u32 { 8_u32 } else { config.top_processes };
    let top_n_usize = top_n as usize;

    top_cpu.truncate(top_n_usize);
    top_mem.truncate(top_n_usize);

    if top_cpu.is_empty() && top_mem.is_empty() {
        add_missing(snapshot, StatId::TopProcesses, MissReason::NoData);
    } else {
        snapshot.top_cpu = top_cpu;
        snapshot.top_mem = top_mem;
    }
}
