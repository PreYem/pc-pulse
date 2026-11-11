//! Text and live rendering for the pcpulse snapshot UI.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

use pcpulse_core::health::{HealthReport, HealthStatus};
use pcpulse_core::model::{Snapshot, StatId};

fn status_label(status: HealthStatus) -> String {
    (
        match status {
            HealthStatus::Healthy => "HEALTHY",
            HealthStatus::Warning => "WARNING",
            HealthStatus::Critical => "CRITICAL",
            HealthStatus::NoData => "NO DATA",
        }
    ).to_string()
}

fn format_one(value: f64) -> String {
    let scaled = (value * 10.0).round() as i64;

    if scaled < 0 && scaled > -10 {
        return format!("-0.{}", -scaled);
    }

    let whole = scaled / 10;
    let frac = if scaled < 0 { -(scaled % 10) } else { scaled % 10 };

    format!("{}.{}", whole, frac)
}

fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes as f64;
    let mut unit = 0_u32;

    while unit < 5_u32 && value >= 1024.0 {
        value /= 1024.0;
        unit += 1;
    }

    format!("{} {}", format_one(value), units[unit as usize])
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400_u64;
    let hours = (seconds % 86400_u64) / 3600_u64;
    let minutes = (seconds % 3600_u64) / 60_u64;
    let secs = seconds % 60_u64;

    if days > 0 {
        return format!("{}d {}h {}m", days, hours, minutes);
    }

    if hours > 0 {
        return format!("{}h {}m", hours, minutes);
    }

    if minutes > 0 {
        return format!("{}m {}s", minutes, secs);
    }

    format!("{}s", seconds)
}

fn missing_reason(snapshot: &Snapshot, id: StatId) -> String {
    for entry in &snapshot.missing {
        if entry.0 == id {
            return entry.1.reason();
        }
    }

    "collector did not report this stat".to_string()
}

fn label_line(snapshot: &Snapshot, id: StatId) -> String {
    format!("{} - N/A - {}", id.label(), missing_reason(snapshot, id))
}

fn snapshot_text(snapshot: &Snapshot, report: &HealthReport) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("pcpulse {}", status_label(report.status.clone())));

    for reason in &report.reasons {
        lines.push(format!("  - {}", reason));
    }

    match snapshot.cpu_count {
        Some(count) => {
            let brand = match snapshot.cpu_brand {
                Some(ref brand) => brand.clone(),
                None => "unknown".to_string(),
            };
            lines.push(format!("CPU - {} / {} cores", brand, count));
        }
        None => lines.push(label_line(snapshot, StatId::CpuUsage))
    }

    if let Some(usage) = snapshot.cpu_total_pct {
        lines.push(format!("CPU usage - {}%", format_one(usage)))
    }

    match snapshot.cpu_freq_max_mhz {
        Some(mhz) => lines.push(format!("CPU frequency - {} MHz max", mhz)),
        None => lines.push(label_line(snapshot, StatId::CpuFreq))
    }

    if !snapshot.cpu_per_core.is_empty() {
        lines.push("Per-core CPU:".to_string());
        for cpu in &snapshot.cpu_per_core {
            lines.push(format!("  {} - {}%", cpu.0, format_one(cpu.1)));
        }
    }

    match snapshot.cpu_temp_c {
        Some(temp) => lines.push(format!("CPU temp - {} C", format_one(temp))),
        None => lines.push(label_line(snapshot, StatId::CpuTemp))
    }

    match snapshot.ram_total_b {
        Some(total) => {
            let used = snapshot.ram_used_b.unwrap_or(0_u64);
            let pct = if total > 0_u64 { used as f64 * 100.0 / total as f64 } else { 0.0 };
            lines.push(format!("RAM - {} / {} used ({}%)", format_bytes(total), format_bytes(used), format_one(pct)));
        }
        None => lines.push(label_line(snapshot, StatId::Ram))
    }

    match snapshot.swap_total_b {
        Some(total) => {
            let used = snapshot.swap_used_b.unwrap_or(0_u64);
            lines.push(format!("Swap - {} / {} used", format_bytes(total), format_bytes(used)));
        }
        None => lines.push(label_line(snapshot, StatId::Swap))
    }

    match snapshot.uptime_s {
        Some(uptime) => lines.push(format!("Uptime - {}", format_uptime(uptime))),
        None => lines.push(label_line(snapshot, StatId::Uptime))
    }

    if snapshot.disks.is_empty() {
        lines.push(label_line(snapshot, StatId::DiskUsage));
    } else {
        for disk in &snapshot.disks {
            let used_pct = if disk.total_b > 0_u64 {
                (disk.total_b - disk.free_b) as f64 * 100.0 / disk.total_b as f64
            } else {
                0.0
            };
            lines.push(format!("Disk {} - {} free of {} ({}% used)", disk.mount, format_bytes(disk.free_b), format_bytes(disk.total_b), format_one(used_pct)));
        }
    }

    if snapshot.net_recv_b.is_some() || snapshot.net_sent_b.is_some() || snapshot.net_errors.is_some() {
        let recv = snapshot.net_recv_b.unwrap_or(0_u64);
        let sent = snapshot.net_sent_b.unwrap_or(0_u64);
        let errors = snapshot.net_errors.unwrap_or(0_u64);
        lines.push(format!("Network - recv {} / sent {} / errors {}", format_bytes(recv), format_bytes(sent), errors));
    } else {
        lines.push(label_line(snapshot, StatId::NetIo));
    }

    if snapshot.top_cpu.is_empty() && snapshot.top_mem.is_empty() {
        lines.push(label_line(snapshot, StatId::TopProcesses));
    } else {
        if !snapshot.top_cpu.is_empty() {
            lines.push("Top CPU:".to_string());
            for proc in &snapshot.top_cpu {
                lines.push(format!("  {} - pid {} - {}% cpu - {}", proc.name, proc.pid, format_one(proc.cpu_pct), format_bytes(proc.mem_b)));
            }
        }

        if !snapshot.top_mem.is_empty() {
            lines.push("Top memory:".to_string());
            for proc in &snapshot.top_mem {
                lines.push(format!("  {} - pid {} - {}", proc.name, proc.pid, format_bytes(proc.mem_b)));
            }
        }
    }

    for entry in &snapshot.missing {
        lines.push(format!("Missing: {} - {}", entry.0.label(), entry.1.reason()));
    }

    let mut out: String = "".to_string();
    let mut first = true;
    for line in &lines {
        if !first {
            out += "\n";
        }
        out += line;
        first = false;
    }

    out
}

pub fn text(snapshot: &Snapshot, report: &HealthReport) -> String {
    snapshot_text(snapshot, report)
}

pub fn draw(snapshot: &Snapshot, report: &HealthReport, frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]);
    let [title_area, body_area, footer_area] = vertical.areas(frame.area());
    let body = Paragraph::new(snapshot_text(snapshot, report));

    frame.render_widget(Paragraph::new("pcpulse".to_string()), title_area);
    frame.render_widget(body, body_area);
    frame.render_widget(Paragraph::new("q quit".to_string()), footer_area);
}