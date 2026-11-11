//! Threshold evaluation.
//!
//! The verdict is intentionally simple: missing stats never turn into fake
//! healthy data; if no monitorable stats are present the status is `NoData`.

use crate::config::Config;
use crate::model::Snapshot;

#[derive(Clone, Debug, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    NoData,
}

pub struct HealthReport {
    pub status: HealthStatus,
    pub reasons: Vec<String>,
}

pub fn evaluate(snapshot: &Snapshot, config: &Config) -> HealthReport {
    let has_data =
        snapshot.cpu_count.is_some() ||
            snapshot.cpu_total_pct.is_some() ||
            snapshot.cpu_temp_c.is_some() ||
            snapshot.ram_total_b.is_some() ||
            snapshot.swap_total_b.is_some() ||
            !snapshot.disks.is_empty() ||
            snapshot.net_recv_b.is_some() ||
            snapshot.net_sent_b.is_some() ||
            snapshot.net_errors.is_some();

    if !has_data {
        return HealthReport {
            status: HealthStatus::NoData,
            reasons: Vec::new(),
        };
    }

    let mut level = 0;
    let mut reasons: Vec<String> = Vec::new();

    if let Some(temp) = snapshot.cpu_temp_c {
        if temp >= config.cpu_temp_bad_c {
            level = 2;
            reasons.push("CPU temperature is critically high.".to_string());
        } else if temp >= config.cpu_temp_warn_c {
            if level < 1 {
                level = 1;
            }
            reasons.push("CPU temperature is high.".to_string());
        }
    }

    if let Some(usage) = snapshot.cpu_total_pct {
        if usage >= config.cpu_usage_bad_pct {
            level = 2;
            reasons.push("CPU usage is critically high.".to_string());
        } else if usage >= config.cpu_usage_warn_pct {
            if level < 1 {
                level = 1;
            }
            reasons.push("CPU usage is high.".to_string());
        }
    }

    if let Some(ram_total) = snapshot.ram_total_b {
        if ram_total > 0 {
            let ram_used = snapshot.ram_used_b.unwrap_or(0);
            let used_pct = ram_used as f64 * 100.0 / ram_total as f64;

            if used_pct >= config.ram_used_bad_pct {
                level = 2;
                reasons.push("RAM usage is critically high.".to_string());
            } else if used_pct >= config.ram_used_warn_pct {
                if level < 1 {
                    level = 1;
                }
                reasons.push("RAM usage is high.".to_string());
            }
        }
    }

    for disk in &snapshot.disks {
        if disk.total_b == 0 {
            continue;
        }

        let free_gb = disk.free_b as f64 / 1073741824.0;

        if free_gb <= config.disk_free_bad_gb {
            level = 2;
            reasons.push("A disk volume is critically low on free space.".to_string());
        } else if free_gb <= config.disk_free_warn_gb {
            if level < 1 {
                level = 1;
            }
            reasons.push("A disk volume is low on free space.".to_string());
        }
    }

    if let Some(errors) = snapshot.net_errors {
        if errors > config.net_error_warn {
            if level < 1 {
                level = 1;
            }
            reasons.push("Network interface reported packet errors.".to_string());
        }
    }

    let status = if level == 2 {
        HealthStatus::Critical
    } else if level == 1 {
        HealthStatus::Warning
    } else {
        HealthStatus::Healthy
    };

    HealthReport {
        status,
        reasons,
    }
}
