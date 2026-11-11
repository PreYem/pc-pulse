//! sysinfo snapshot collector.
//!
//! One-shot collection is used both by `--snapshot-once` and by the live UI.
//! Missing values are explicit `None` fields and are accompanied by an entry in
//! `missing` so the UI can display an honest reason instead of inventing data.

use crate::config::{Config, default_config};
use crate::model::{DiskStat, MissReason, ProcStat, Snapshot, StatId};

use std::cmp::Ordering;
use std::thread::sleep;

use sysinfo::{
    Components,
    Disks,
    MINIMUM_CPU_UPDATE_INTERVAL,
    Networks,
    System,
};

fn add_missing(snapshot: &mut Snapshot, id: StatId, reason: MissReason) {
    for entry in &snapshot.missing {
        if entry.0 == id {
            return;
        }
    }

    snapshot.missing.push((id, reason));
}

fn basic_ids() -> [StatId; 9] {
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

fn not_implemented_ids() -> [StatId; 6] {
    [
        StatId::GpuTemp,
        StatId::GpuUtil,
        StatId::GpuVram,
        StatId::FanSpeed,
        StatId::PowerDraw,
        StatId::SsdSmart,
    ]
}

fn all_missing(reason: MissReason) -> Snapshot {
    let mut snapshot = Snapshot::default();

    for id in basic_ids() {
        add_missing(&mut snapshot, id, reason.clone());
    }

    for id in not_implemented_ids() {
        add_missing(&mut snapshot, id, MissReason::CollectorNotImplemented);
    }

    snapshot
}

fn cpu_temp_label_matches(label: String) -> bool {
    let needle = label.to_lowercase();
    needle.contains("cpu") ||
        needle.contains("core") ||
        needle.contains("physical") ||
        needle.contains("operating") ||
        needle.contains("processor") ||
        needle.contains("temperature")
}

fn cmp_cpu_desc(a: &ProcStat, b: &ProcStat) -> Ordering {
    b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(Ordering::Equal)
}

fn cmp_mem_desc(a: &ProcStat, b: &ProcStat) -> Ordering {
    b.mem_b.cmp(&a.mem_b)
}

pub fn collect_once(config: &Config) -> Snapshot {
    let mut snapshot = Snapshot::default();

    match System::new_all() {
        Ok(mut sys) => {
            sys.refresh_cpu_all();
            sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu_all();

            let cpus = sys.cpus();

            if cpus.is_empty() {
                add_missing(&mut snapshot, StatId::CpuUsage, MissReason::NoData);
                add_missing(&mut snapshot, StatId::CpuFreq, MissReason::NoData);
            } else {
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
                    add_missing(&mut snapshot, StatId::CpuFreq, MissReason::NoData);
                } else {
                    snapshot.cpu_freq_max_mhz = Some(max_freq);
                }
            }

            match System::uptime() {
                Ok(uptime_s) => {
                    snapshot.uptime_s = Some(uptime_s);
                }
                Err(_) => {
                    add_missing(&mut snapshot, StatId::Uptime, MissReason::CollectorError);
                }
            }

            let ram_total = sys.total_memory();
            if ram_total == 0 {
                add_missing(&mut snapshot, StatId::Ram, MissReason::NoData);
            } else {
                snapshot.ram_total_b = Some(ram_total);
                snapshot.ram_used_b = Some(sys.used_memory());
            }

            let swap_total = sys.total_swap();
            if swap_total == 0 {
                add_missing(&mut snapshot, StatId::Swap, MissReason::NoData);
            } else {
                snapshot.swap_total_b = Some(swap_total);
                snapshot.swap_used_b = Some(sys.used_swap());
            }

            match Disks::new_with_refreshed_list() {
                Ok(disks) => {
                    let mut count = 0;

                    for disk in &disks {
                        count += 1;

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

                    if count == 0 {
                        add_missing(&mut snapshot, StatId::DiskUsage, MissReason::NoData);
                    }
                }
                Err(_) => {
                    add_missing(&mut snapshot, StatId::DiskUsage, MissReason::CollectorError);
                }
            }

            match Networks::new_with_refreshed_list() {
                Ok(networks) => {
                    let mut count = 0;
                    let mut recv = 0_u64;
                    let mut sent = 0_u64;
                    let mut errors = 0_u64;

                    for (_, network) in &networks {
                        count += 1;
                        recv += network.total_received();
                        sent += network.total_transmitted();
                        errors += network.total_errors_on_received();
                        errors += network.total_errors_on_transmitted();
                    }

                    if count == 0 || (recv == 0 && sent == 0 && errors == 0) {
                        add_missing(&mut snapshot, StatId::NetIo, MissReason::NoData);
                    } else {
                        snapshot.net_recv_b = Some(recv);
                        snapshot.net_sent_b = Some(sent);
                        snapshot.net_errors = Some(errors);
                    }
                }
                Err(_) => {
                    add_missing(&mut snapshot, StatId::NetIo, MissReason::CollectorError);
                }
            }

            match Components::new_with_refreshed_list() {
                Ok(components) => {
                    let mut max_temp = -1000.0;
                    let mut seen = 0;

                    for component in &components {
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
                        add_missing(&mut snapshot, StatId::CpuTemp, MissReason::NoData);
                    } else {
                        snapshot.cpu_temp_c = Some(max_temp);
                    }
                }
                Err(_) => {
                    add_missing(&mut snapshot, StatId::CpuTemp, MissReason::CollectorError);
                }
            }

            if !config.show_per_core {
                snapshot.cpu_per_core.clear();
            }

            let mut top_cpu: Vec<ProcStat> = Vec::new();
            let mut top_mem: Vec<ProcStat> = Vec::new();

            for proc in sys.processes().values() {
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
                add_missing(&mut snapshot, StatId::TopProcesses, MissReason::NoData);
            } else {
                snapshot.top_cpu = top_cpu;
                snapshot.top_mem = top_mem;
            }
        }
        Err(_) => {
            return all_missing(MissReason::CollectorError);
        }
    }

    for id in not_implemented_ids() {
        add_missing(&mut snapshot, id, MissReason::CollectorNotImplemented);
    }

    snapshot
}

pub fn collect_snapshot_once() -> Snapshot {
    collect_once(&default_config())
}
