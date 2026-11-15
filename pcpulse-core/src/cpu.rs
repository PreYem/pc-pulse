//! CPU category probes: usage / per-core / frequency / brand / count, and
//! best-effort CPU temperature from sysinfo `Components`.

use crate::config::Config;
use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

fn cpu_temp_label_matches(label: String) -> bool {
    let needle = label.to_lowercase();
    needle.contains("cpu") ||
        needle.contains("core") ||
        needle.contains("physical") ||
        needle.contains("operating") ||
        needle.contains("processor") ||
        needle.contains("temperature")
}

pub fn probe_cpu(snapshot: &mut Snapshot, ctx: &ProbeCtx, config: &Config) {
    let cpus = ctx.system.cpus();
    snapshot.cpu_per_core.clear();

    if cpus.is_empty() {
        add_missing(snapshot, StatId::CpuUsage, MissReason::NoData);
        add_missing(snapshot, StatId::CpuFreq, MissReason::NoData);
        return;
    }

    let mut sum_usage = 0.0;
    let mut max_freq = 0_u64;

    for cpu in cpus {
        let usage = cpu.usage() as f64;
        sum_usage += usage;

        if cpu.frequency() > max_freq {
            max_freq = cpu.frequency();
        }

        snapshot.cpu_per_core.push((cpu.name().to_string(), usage));
    }

    snapshot.cpu_count = Some(cpus.len() as u32);
    snapshot.cpu_total_pct = Some(sum_usage / cpus.len() as f64);

    if let Some(first) = cpus.first() {
        snapshot.cpu_brand = Some(first.brand().to_string());
    }

    if max_freq == 0 {
        add_missing(snapshot, StatId::CpuFreq, MissReason::NoData);
    } else {
        snapshot.cpu_freq_max_mhz = Some(max_freq);
    }

    if !config.show_per_core {
        snapshot.cpu_per_core.clear();
    }
}

pub fn probe_cpu_temp(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let mut max_temp = -1000.0;
    let mut seen = 0;

    for component in &ctx.components {
        if let Some(raw_temp) = component.temperature() {
            let temp = raw_temp as f64;

            if temp > -1000.0 &&
                temp < 1000.0 &&
                cpu_temp_label_matches(component.label().to_string()) {
                seen += 1;

                if temp > max_temp {
                    max_temp = temp;
                }
            }
        }
    }

    if seen == 0 {
        add_missing(snapshot, StatId::CpuTemp, MissReason::NoData);
    } else {
        snapshot.cpu_temp_c = Some(max_temp);
    }
}
