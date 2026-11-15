//! GPU category: identity (vendor / name / total VRAM) and utilization.
//!
//! Windows uses DXGI for identity (documented, user-mode) and the optional LHM
//! web server (`lhm.rs`) for utilization/temperature/VRAM-used/fan/power/SMART,
//! because DXGI in user-mode does not expose those percentages. Linux and macOS
//! use the sysinfo `Gpus` backends (safe mode only): identity and utilization
//! come from sysinfo, while temperature/fan/power/SMART stay `N/A` (`NotSupported`)
//! until the experimental `/sys` + hwmon + SMART path lands in plan #8.

use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory;

#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::IDXGIFactory;

#[cfg(windows)]
use std::char::from_u32;

#[cfg(windows)]
#[derive(Clone, Debug, Default)]
pub struct GpuInfo {
    pub vendor: String,
    pub name: String,
    pub vram_total_b: u64,
}

#[cfg(windows)]
fn vendor_name(vendor_id: u32) -> String {
    (
        match vendor_id {
            0x10de => "NVIDIA".to_string(),
            0x1002 => "AMD".to_string(),
            0x8086 => "Intel".to_string(),
            0x1005 => "Conduit".to_string(),
            _ => format!("vendor 0x{0:x}", vendor_id),
        }
    ).to_string()
}

#[cfg(windows)]
fn decode_description(raw: &[u16]) -> String {
    let mut chars: Vec<char> = Vec::new();

    for unit in raw {
        if *unit == 0_u16 {
            break;
        }

        if let Some(ch) = from_u32(*unit as u32) {
            chars.push(ch);
        }
    }

    let text: String = chars.iter().collect();
    text
}

#[cfg(windows)]
fn dxgi_probe() -> GpuInfo {
    let factory = unsafe { CreateDXGIFactory::<IDXGIFactory>() };

    match factory {
        Ok(factory) => {
            let mut index = 0_u32;

            while index < 16_u32 {
                match unsafe { factory.EnumAdapters(index) } {
                    Ok(adapter) => {
                        if let Ok(desc) = unsafe { adapter.GetDesc() } {
                            let name = decode_description(&desc.Description);

                            if name != "Generic Display Adapter" {
                                let info = GpuInfo {
                                    vendor: vendor_name(desc.VendorId),
                                    name: name.to_string(),
                                    vram_total_b: desc.DedicatedVideoMemory as u64,
                                };

                                return info;
                            }
                        }
                    },
                    Err(_) => {
                        break;
                    },
                }

                index += 1;
            }

            GpuInfo::default()
        },
        Err(_) => {
            GpuInfo::default()
        },
    }
}

#[cfg(windows)]
pub fn probe_identity(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let _ctx = ctx;
    let gpu = dxgi_probe();

    if !gpu.vendor.is_empty() || !gpu.name.is_empty() {
        snapshot.gpu_vendor = Some(gpu.vendor.to_string());
        snapshot.gpu_name = Some(gpu.name.to_string());

        if gpu.vram_total_b > 0_u64 {
            snapshot.gpu_vram_total_b = Some(gpu.vram_total_b);
        } else {
            add_missing(snapshot, StatId::GpuVram, MissReason::NoData);
        }
    } else {
        add_missing(snapshot, StatId::GpuVram, MissReason::CollectorError);
    }
}

#[cfg(not(windows))]
use sysinfo::Gpu;

#[cfg(not(windows))]
fn pick_primary_gpu(ctx: &ProbeCtx) -> Option<&Gpu> {
    let mut fallback: Option<&Gpu> = None;

    for gpu in &ctx.gpus {
        if gpu.total_memory().unwrap_or(0_u64) > 0_u64 {
            return Some(gpu);
        }

        if fallback.is_none() {
            fallback = Some(gpu);
        }
    }

    fallback
}

#[cfg(not(windows))]
pub fn probe_identity(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let best = pick_primary_gpu(ctx);

    match best {
        Some(gpu) => {
            match gpu.vendor() {
                Some(vendor) => snapshot.gpu_vendor = Some(vendor.to_string()),
                None => {},
            }

            match gpu.model() {
                Some(model) => snapshot.gpu_name = Some(model.to_string()),
                None => {},
            }

            match gpu.total_memory() {
                Some(total) => snapshot.gpu_vram_total_b = Some(total),
                None => add_missing(snapshot, StatId::GpuVram, MissReason::NoData),
            }
        },
        None => {
            add_missing(snapshot, StatId::GpuVram, MissReason::NoData);
        },
    }
}

#[cfg(not(windows))]
pub fn probe_gpu_util(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let gpu = pick_primary_gpu(ctx);

    match gpu {
        Some(gpu) => {
            match gpu.usage() {
                Some(usage) => snapshot.gpu_util_pct = Some(usage as f64),
                None => add_missing(snapshot, StatId::GpuUtil, MissReason::NoData),
            }

            match gpu.used_memory() {
                Some(used) => snapshot.gpu_vram_used_b = Some(used),
                None => {},
            }
        },
        None => {
            add_missing(snapshot, StatId::GpuUtil, MissReason::NoData);
        },
    }
}
