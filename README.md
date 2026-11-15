# pcpulse

`pcpulse` is a Rust-based whole-PC health monitor. The current phase provides a terminal app for Windows 10 that collects CPU, RAM, swap, uptime, disk, network, and top-process stats, GPU identity via DXGI, and an opt-in LibreHardwareMonitor web-server feed for GPU utilization/temperature/VRAM-used/fan/power/SMART, plus a simple health verdict. The live UI refreshes HOT probes every 500 ms and COLD probes every ~30 s.

Missing or unsupported sensors are shown as `N/A` with a reason. The project does not fake values or zero-fill unavailable data.

## Current status

- Phase 0: planning and environment checks are done.
- Phase 1: `pcpulse-core` and `pcpulse-tui` are implemented and compiling.
- Phase 2: GPU identity via DXGI and the opt-in LHM JSON collector are implemented.
- Phase 3: per-stat probe registry + HOT/COLD live collector are implemented and compiling.
- The current release binary `pcpulse-tui.exe` builds and can print a one-time snapshot or run the live TUI.
- Later phases are still planned:
  - Windows advanced stats: ADLX etc.
  - Linux/macOS parity where possible (safe mode: sysinfo backends only; experimental `/sys` + hwmon + SMART deferred to plan #8).
  - GUI front-end with tray/minimized launch.
  - Windows installer.
  - CI/release artifacts.

## Clone / pull

The GitHub repository is:

```text
https://github.com/PreYem/pc-pulse.git
```

Clone it:

```powershell
git clone https://github.com/PreYem/pc-pulse.git
cd pc-pulse
```

If working in an existing checkout:

```powershell
git pull
```

## Requirements

For Windows 10 builds:

- Rust toolchain: `rustc` and `cargo`
- MSVC linker / Visual Studio Build Tools toolchain
- No admin privileges are required for the current stats

## Build

From the repository root:

```powershell
cargo build --workspace
cargo build --workspace --release
```

Quality checks:

```powershell
cargo check --workspace
cargo clippy --workspace --all-targets -D warnings
```

The current phase also verified:

```powershell
cargo build --package pcpulse-tui --release
```

## Run

Snapshot mode:

```powershell
target\release\pcpulse-tui.exe --snapshot-once
```

Live terminal UI:

```powershell
target\release\pcpulse-tui.exe
```

Quit live mode with:

```text
q
```

Other flags:

```powershell
target\release\pcpulse-tui.exe --help
target\release\pcpulse-tui.exe --version
```

Debug builds can be run from `target\debug\` using the same commands.

## Install the TUI binary

You can install it with Cargo:

```powershell
cargo install --path pcpulse-tui
```

That usually places `pcpulse-tui.exe` in Cargo's binary directory. After installing, run:

```powershell
pcpulse-tui.exe --snapshot-once
```

Or manually copy/rename the release binary:

```powershell
copy target\release\pcpulse-tui.exe pcpulse.exe
```

Then place `pcpulse.exe` in a directory on your `PATH`.

## Configuration

`pcpulse-core` creates a user-editable config file in the platform config directory if none exists. On Windows this is usually:

```text
%LOCALAPPDATA%\pcpulse\config.toml
```

The default template lives in the repository at:

```text
pcpulse-core\default.toml
```

Thresholds affect the health verdict:
- `Good`
- `Caution`
- `Critical`

Missing stats are neutral: they are shown as `N/A` with the reason and never count as healthy.

## Project layout

```text
pcpulse/
  pcpulse-core/   library: stats model, registry/probes, sysinfo collector, health, config
  pcpulse-tui/    terminal binary: snapshot CLI and live TUI
  .opencode/      project config + tracked context.md; plans/ (git-ignored numbered plans)
  scripts/        dependency and environment helper scripts
```

## Tracked vs ignored files

Important committed files:
- source code
- `Cargo.toml`
- `Cargo.lock`
- `.gitignore`
- `README.md`
- `.opencode/context.md`
- scripts

Ignored by default:
- `target/`
- local working plans except `.opencode/context.md`
- build caches
- OS/editor artifacts
- `.env` files if present

## Current sensor honesty

Currently available on Windows 10:
- CPU usage, per-core usage, process/core counts, max CPU frequency
- RAM used/total/percent
- swap/pagefile
- uptime
- disk free/total/used percent
- network totals
- top CPU and top memory processes
- GPU vendor/name/total VRAM via DXGI
- best-effort CPU temp if a user-mode data source is available

Currently shown as `N/A` with reasons:
- GPU utilization, GPU temp, fan speeds, power draw, VRAM used, SSD SMART — require the LibreHardwareMonitor web server (`NeedsLhm` when `[lhm] enabled = false`)
- CPU temp when no user-mode data source is available
