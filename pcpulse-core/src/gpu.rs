//! Windows GPU identity: DXGI adapter enumeration (documented, user-mode).
//!
//! Reports vendor, adapter name and total dedicated VRAM. Percentages
//! (GPU utilization, used VRAM, temperature, fan speed, power) are not
//! exposed by DXGI in user-mode; those come from the optional LHM web
//! server (see `lhm.rs`) or render as N/A with an honest reason.

use windows::Win32::Graphics::Dxgi::CreateDXGIFactory;
use windows::Win32::Graphics::Dxgi::IDXGIFactory;

use std::char::from_u32;

#[derive(Clone, Debug, Default)]
pub struct GpuInfo {
    pub vendor: String,
    pub name: String,
    pub vram_total_b: u64,
}

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

pub fn probe() -> GpuInfo {
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
