# pcpulse — living project context

LIVING FILE — update this after every phase/plan step (status, layout, build state, next steps).
Tracked in git; keep it accurate. Numbered plans are working notes under `.opencode/plans/` and are git-ignored.

## Plan-folder rule

- Numbered plans live at `.opencode/plans/#1.md`, `.opencode/plans/#2.md`, … (git-ignored).
- `.opencode/context.md` is the only tracked context/plan-folder doc.
- New plans append the next number; update this file in the same session that executes a phase.

## Read before working

1. This file: `.opencode/context.md`
2. `README.md`
3. The most recent numbered plan in `.opencode/plans/`
4. Relevant source files before editing

## Project

`pcpulse` is a whole-PC health monitor in Rust (Windows 10 first; Linux/macOS parity later).
Collector library `pcpulse-core` + front-ends; stats are `Option<T>` + `Unavailable(reason)`,
rendered as `N/A — <reason>`; never fake or zero-fill sensor data.

## Phase status (update this section as work progresses)

- Phase 0 — planning, environment checks, dependency approval (plan #1): DONE.
- Phase 1 — `pcpulse-core` sysinfo stats, `pcpulse-tui` snapshot/live mode, health verdict,
  config fallback, CLI flags: DONE (strict clippy + release build confirmed during Phase 2 session).
- Phase 2 — GPU identity via DXGI (`gpu.rs`), opt-in LHM JSON (`lhm.rs`, `[lhm]` config, port 24817
  default), GPU model/config/health/TUI wiring: DONE (clippy clean, release build OK,
  `--snapshot-once` on real hardware reports AMD Radeon RX 7800 XT / 16 GB VRAM; LHM rows render
  as N/A with the `NeedsLhm` reason when disabled).
- Not started: remaining Windows stats paths (ADLX etc.), Linux/macOS parity,
  GUI app (`pcpulse-gui` + tray), Inno Setup installer, CI/release pipelines (plans #2+ not written yet).

## Workspace layout

```text
pcpulse/
  pcpulse-core/   lib: model, sysinfo collector, health, config (default.toml template)
  pcpulse-tui/    bin: ratatui snapshot CLI + live TUI
  pcpulse-gui/    planned: eframe/egui + tray-icon front-end
  installer/      planned: Inno Setup pcpulse.iss
  scripts/        check-deps.ps1 (read-only env checks)
  .opencode/
    context.md    this tracked living context file (wired via opencode.json instructions)
    opencode.json project config (tracked)
    plans/        git-ignored numbered plans: unfinished at plans/ (#4-Linux-MacOS-Parity,
                  #5-GUI-Tray-Autostart, #6-Inno-Setup-Installer, #7-CI-Release-Builds);
                  finished moved to plans/DONE/ (#1-Setup-Planning, #2-Core-TUI-Basics,
                  #3-GPU-LHM-Stats) 2026-11-15 (AGENTS.md moved here 2026-11-11)
    skills/       project skills copied from Anthropic claude-skills (tracked)
```

## Current build status

- Workspace debug build + strict clippy (`-- -D warnings`) + full release build: OK.
- `pcpulse-tui --snapshot-once`: GPU identity verified on real hardware (2026-11-15).
- `windows` crate dependency is unconditional (`#[cfg(windows)]` crate); must be made
  conditional (target-gated) before Linux/macOS CI builds.
- `target\release\pcpulse-tui.exe --snapshot-once`: tested, works.
- Config: `%LOCALAPPDATA%\pcpulse\config.toml` created from `pcpulse-core\default.toml` template.

## Useful commands

```powershell
cargo check --workspace
cargo clippy --workspace --all-targets -D warnings
cargo build --workspace
cargo build --workspace --release
cargo build --package pcpulse-tui --release
target\release\pcpulse-tui.exe --snapshot-once
```

## Approved crates (from plan #1 — don't add others without asking)

sysinfo 0.40.1 · ratatui 0.30.2 · crossterm 0.29.0 · eframe/egui 0.37.0 · tray-icon 0.26.1 ·
auto-launch 0.6.0 · directories 6.0.0 · ureq 3.5.1 · toml 1.2.1 (serde+display) · serde/serde_json ·
anyhow 1.0.104 · windows 0.62.2 (Dxgi/Dxgi_Common/Direct3D11, System_Registry only) ·
windows_exe_info 0.5.3 (build-dep, Phase 5). LHM web-server fallback is opt-in; suggested port 24817.

## Project rules

- Do not add unapproved dependencies without asking.
- Missing stats render as `N/A` with an honest reason; never zero-fill.
- Do not start network/LHM servers without asking for port permission (use uncommon ports, `--strictPort`).
- Never delete files/dirs without permission; never run git write commands unless explicitly requested.
- After each phase: `cargo clippy --workspace --all-targets -D warnings`, then release build; report DONE + changes.
- Keep `.opencode/context.md` current at the end of every session that changes the project.

## Session log

- 2026-11-10: Plan #1 drafted; Phase 0 env checks passed; blockers resolved.
- 2026-11-10/11: Phase 1 implemented (`pcpulse-core` + `pcpulse-tui`); TUI release snapshot tested.
- 2026-11-11: Plans folder moved to `.opencode/plans/`; `.gitignore` updated (old `plans/*` rule
  replaced with `.opencode/plans/*`); context moved from `plans/AGENTS.md` to this tracked file;
  skills `doc-coauthoring`, `frontend-design`, `skill-creator` copied from
  `D:\Projects\claude-skills` into `.opencode/skills/`. Commit still pending (user will commit).
- 2026-11-15: Phase 2 — added `gpu.rs` (DXGI `CreateDXGIFactory`/`EnumAdapters`/`GetDesc`; vendor IDs
  NVIDIA/AMD/Intel/Conduit; skips "Generic Display Adapter"), `lhm.rs` (opt-in `ureq` GET of
  `data.json`, `serde_json::Value` walk: gpuload/gputemperature/gpumemload/gpupower/fan/smarthdd),
  `MissReason::NeedsLhm`, GPU thresholds in config/default.toml, nested `[lhm]` with serde
  default fns, GPU/LHM merge in `collector.rs`, GPU temp/util checks in `health.rs`, GPU rows in
  `view.rs`. WDK/D3DKMT memory-segment APIs unavailable in enabled modules → DXGI
  `DedicatedVideoMemory` used for total VRAM instead. Strict clippy + release build clean.
- 2026-11-15: Phase 2 experiments concluded — PDH GPU-Engine: ntdll.dll has NO named `PDH*`
  exports on this build (ordinal-only) → GPU util LHM fallback; ADLX: `amdadlx64.dll` present in
  System32 but SDK function names absent from exports (ordinal-only) → temp/power/fan LHM-only.
  Plan #3 now DONE.
- 2026-11-15: Plan #1 split per-phase into `.opencode/plans/`; files then renamed with 1–4-word
  names + `DONE` suffix for finished phases (`#1-Setup-Planning-DONE`, `#2-Core-TUI-Basics-DONE`,
  `#3-GPU-LHM-Stats` PARTIAL, `#4-Linux-MacOS-Parity`, `#5-GUI-Tray-Autostart`,
  `#6-Inno-Setup-Installer`, `#7-CI-Release-Builds` — last four NOT STARTED).
- 2026-11-15: Finished plans #1-#3 moved to `.opencode/plans/DONE/` (suffixes dropped);
  unfinished #4-#7 stay at `.opencode/plans/`.
- 2026-11-15: User decisions recorded in plan files: live-collector = 500 ms tick with HOT/COLD
  probe groups (redesign in plan #4); Linux parity = safe mode (sysinfo only), experimental /sys
  parsing deferred to new plan #8 after local Linux test (WSL2/VM); Sensor-trait replaced by
  per-stat probe functions + central registry (plan #4); version single-source from
  Cargo.toml noted in plan #6. LHM "how to enable" docs question still unanswered.
- 2026-11-15: New user request → plan #5 section: bundle LibreHardwareMonitor (MPL-2.0) in
  installer, auto-spawn detached/minimized with web server on 24817 at pcpulse launch, kill
  on exit; README gets a short LHM section (doc question answered YES).
