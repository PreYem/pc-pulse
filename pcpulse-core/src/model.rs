//! Snapshot model: every stat is optional; missing stats carry a reason.
//!
//! The TUI renders every `None` field as "N/A — <reason>" so the user always
//! knows why a value is absent. Values are never faked or zero-filled.

#[derive(Clone, Debug, PartialEq)]
pub enum StatId {
    Uptime,
    CpuUsage,
    CpuFreq,
    CpuTemp,
    Ram,
    Swap,
    DiskUsage,
    NetIo,
    TopProcesses,
    GpuTemp,
    GpuUtil,
    GpuVram,
    FanSpeed,
    PowerDraw,
    SsdSmart,
}

impl StatId {
    pub fn label(&self) -> String {
        (
            match self {
                StatId::Uptime => "Uptime",
                StatId::CpuUsage => "CPU usage",
                StatId::CpuFreq => "CPU frequency",
                StatId::CpuTemp => "CPU temp",
                StatId::Ram => "RAM",
                StatId::Swap => "Swap",
                StatId::DiskUsage => "Disk usage",
                StatId::NetIo => "Network",
                StatId::TopProcesses => "Top processes",
                StatId::GpuTemp => "GPU temp",
                StatId::GpuUtil => "GPU usage",
                StatId::GpuVram => "GPU VRAM",
                StatId::FanSpeed => "Fan speed",
                StatId::PowerDraw => "Power draw",
                StatId::SsdSmart => "SSD SMART",
            }
        ).to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MissReason {
    NeedsAdmin,
    NotSupported,
    CollectorNotImplemented,
    CollectorError,
    NoData,
}

impl MissReason {
    pub fn reason(&self) -> String {
        (
            match self {
                MissReason::NeedsAdmin => "requires administrator privileges",
                MissReason::NotSupported => "not supported on this platform",
                MissReason::CollectorNotImplemented => "collector not implemented yet (Phase 2/3)",
                MissReason::CollectorError => "collector reported an error",
                MissReason::NoData => "no data available on this machine",
            }
        ).to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiskStat {
    pub mount: String,
    pub total_b: u64,
    pub free_b: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcStat {
    pub name: String,
    pub pid: u32,
    pub cpu_pct: f64,
    pub mem_b: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub cpu_brand: Option<String>,
    pub cpu_count: Option<u32>,
    pub cpu_total_pct: Option<f64>,
    pub cpu_per_core: Vec<(String, f64)>,
    pub cpu_freq_max_mhz: Option<u64>,
    pub cpu_temp_c: Option<f64>,

    pub ram_total_b: Option<u64>,
    pub ram_used_b: Option<u64>,
    pub swap_total_b: Option<u64>,
    pub swap_used_b: Option<u64>,

    pub uptime_s: Option<u64>,

    pub disks: Vec<DiskStat>,

    pub net_recv_b: Option<u64>,
    pub net_sent_b: Option<u64>,
    pub net_errors: Option<u64>,

    pub top_cpu: Vec<ProcStat>,
    pub top_mem: Vec<ProcStat>,

    pub missing: Vec<(StatId, MissReason)>,
}
