//! Opt-in LibreHardwareMonitor web-server JSON collection.
//!
//! Gated by the `[lhm]` config block: with `enabled = false` no HTTP request
//! is made and the dependent stats stay N/A with an honest reason. The app
//! only ever reads an already-running server; it never starts one.

use serde_json::Value;

use crate::config::Config;
use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::{add_missing, lhm_ids};

#[derive(Clone, Debug, Default)]
pub struct LhmStats {
    pub gpu_util_pct: Option<f64>,
    pub gpu_temp_c: Option<f64>,
    pub gpu_vram_used_pct: Option<f64>,
    pub gpu_power_w: Option<f64>,
    pub fans: Vec<(String, f64)>,
    pub smart: Vec<(String, u64)>,
}

fn string_at(item: &Value, key: String) -> String {
    match item.as_object() {
        Some(obj) => {
            match obj.get(&key) {
                Some(found) => {
                    match found.as_str() {
                        Some(text) => text.to_string(),
                        None => "".to_string(),
                    }
                },
                None => "".to_string(),
            }
        },
        None => "".to_string(),
    }
}

fn number_at(item: &Value, key: String) -> Option<f64> {
    match item.as_object() {
        Some(obj) => {
            match obj.get(&key) {
                Some(found) => found.as_f64(),
                None => None,
            }
        },
        None => None,
    }
}

fn apply_item(stats: &mut LhmStats, item: &Value) {
    let kind = string_at(item, "type".to_string()).to_lowercase();
    let name = string_at(item, "name".to_string());

    match kind.as_str() {
        "gpuload" => {
            if stats.gpu_util_pct.is_none() {
                stats.gpu_util_pct = number_at(item, "value_float".to_string());
            }
        },
        "gputemperature" => {
            stats.gpu_temp_c = number_at(item, "value_float".to_string());
        },
        "gpucoretemp" => {
            if stats.gpu_temp_c.is_none() {
                stats.gpu_temp_c = number_at(item, "value_float".to_string());
            }
        },
        "gpumemload" => {
            stats.gpu_vram_used_pct = number_at(item, "value_float".to_string());
        },
        "gpucorememload" => {
            if stats.gpu_vram_used_pct.is_none() {
                stats.gpu_vram_used_pct = number_at(item, "value_float".to_string());
            }
        },
        "gpupower" => {
            stats.gpu_power_w = number_at(item, "value_float".to_string());
        },
        "fan" => {
            if let Some(value) = number_at(item, "value_float".to_string()) {
                stats.fans.push((name.to_string(), value));
            }
        },
        "smarthdd" => {
            if let Some(value) = number_at(item, "value_int".to_string()) {
                if (0.0..=100.0).contains(&value) {
                    stats.smart.push((name.to_string(), value as u64));
                }
            }
        },
        _ => {},
    }
}

pub fn collect(url: String) -> (LhmStats, Option<String>) {
    let response = ureq::get(&url).call();

    let text = match response {
        Ok(mut response) => {
            match response.body_mut().read_to_string() {
                Ok(text) => text,
                Err(error) => {
                    return (LhmStats::default(), Some(format!("LHM response could not be read: {}", error)));
                },
            }
        },
        Err(error) => {
            return (LhmStats::default(), Some(format!("LHM request failed: {}", error)));
        },
    };

    let value = match serde_json::from_str::<Value>(&text) {
        Ok(value) => value,
        Err(error) => {
            return (LhmStats::default(), Some(format!("LHM JSON invalid: {}", error)));
        },
    };

    let mut stats = LhmStats::default();

    match value.as_object() {
        Some(obj) => {
            match obj.get(&"items".to_string()) {
                Some(items_value) => {
                    match items_value.as_array() {
                        Some(items) => {
                            for item in items {
                                apply_item(&mut stats, item);
                            }
                        },
                        None => {
                            return (LhmStats::default(), Some("LHM items array missing".to_string()));
                        },
                    }
                },
                None => {
                    return (LhmStats::default(), Some("LHM items field missing".to_string()));
                },
            }
        },
        None => {
            return (LhmStats::default(), Some("LHM response is not a JSON object".to_string()));
        },
    }

    (stats, None)
}

#[cfg(windows)]
pub fn probe_lhm(snapshot: &mut Snapshot, config: &Config) {
    if !config.lhm.enabled {
        for id in lhm_ids() {
            add_missing(snapshot, id, MissReason::NeedsLhm);
        }

        return;
    }

    let lhm_result = collect(config.lhm.url.clone());
    let stats: LhmStats = lhm_result.0;
    let error = lhm_result.1;

    match error {
        Some(message) => {
            let _message = message;

            for id in lhm_ids() {
                add_missing(snapshot, id, MissReason::CollectorError);
            }
        },
        None => {
            match stats.gpu_util_pct {
                Some(pct) => snapshot.gpu_util_pct = Some(pct),
                None => add_missing(snapshot, StatId::GpuUtil, MissReason::NoData),
            }

            match stats.gpu_temp_c {
                Some(temp) => snapshot.gpu_temp_c = Some(temp),
                None => add_missing(snapshot, StatId::GpuTemp, MissReason::NoData),
            }

            match stats.gpu_vram_used_pct {
                Some(pct) => {
                    match snapshot.gpu_vram_total_b {
                        Some(total) => {
                            if pct > 0.0 && pct <= 100.0 {
                                snapshot.gpu_vram_used_b = Some((total as f64 * pct / 100.0) as u64);
                            }
                        },
                        None => {
                            add_missing(snapshot, StatId::GpuVram, MissReason::NoData);
                        },
                    }
                },
                None => {
                    if snapshot.gpu_vram_total_b.is_some() {
                        add_missing(snapshot, StatId::GpuVram, MissReason::NeedsLhm);
                    }
                },
            }

            match stats.gpu_power_w {
                Some(power) => snapshot.gpu_power_w = Some(power),
                None => add_missing(snapshot, StatId::PowerDraw, MissReason::NoData),
            }

            if stats.fans.is_empty() {
                add_missing(snapshot, StatId::FanSpeed, MissReason::NoData);
            } else {
                snapshot.gpu_fans = stats.fans;
            }

            if stats.smart.is_empty() {
                add_missing(snapshot, StatId::SsdSmart, MissReason::NoData);
            } else {
                snapshot.ssd_smart = stats.smart;
            }
        },
    }
}

#[cfg(not(windows))]
pub fn probe_lhm(snapshot: &mut Snapshot, config: &Config) {
    let _config = config;

    for id in lhm_ids() {
        add_missing(snapshot, id, MissReason::NotSupported);
    }
}
