# pcpulse project context

This is the tracked context file for the `pcpulse` project.

## Plan-folder rule

- Only `plans/AGENTS.md` should be tracked by git.
- Other files under `plans/` are working notes and are ignored.
- Numbered plans use names like `plans/#1.md`, `plans/#2.md`, etc.

## Read before working

1. This file: `plans/AGENTS.md`
2. `README.md` if present
3. The most recent numbered plan file in `plans/`
4. Relevant source files before editing

## Project

`pcpulse` is a whole-PC health monitor in Rust.

Current phase status:

- Phase 0: planning, environment checks, dependency approval. Status: done.
- Phase 1: `pcpulse-core` sysinfo stats, `pcpulse-tui` snapshot/live mode, health verdict, config fallback, CLI flags. Status: mostly complete.
- Verification gap: strict clippy with `-D warnings` and full workspace release build still need confirmation.
- Later phases are not executed yet: advanced Windows stats, platform parity, GUI app, installer, CI/release pipelines.

## Workspace layout

```text
pcpulse/
  pcpulse-core/   Rust library: model, collector, health, config
  pcpulse-tui/    Terminal app: snapshot CLI and live TUI
  pcpulse-gui/    Planned future eframe/egui front-end
  installer/      Planned future Inno Setup scripts
  scripts/        Utility scripts
  plans/          Numbered plans and tracked AGENTS.md
```

## Current build status

- Debug workspace build succeeded.
- `pcpulse-tui` release build succeeded.
- `target\release\pcpulse-tui.exe --snapshot-once` was tested and works.
- `target\release\pcpulse-tui.exe` is the current installable binary for the TUI phase.

## Useful commands

```powershell
cargo check --workspace
cargo clippy --workspace --all-targets -D warnings
cargo build --workspace
cargo build --workspace --release
cargo build --package pcpulse-tui --release
target\release\pcpulse-tui.exe --snapshot-once
target\release\pcpulse-tui.exe --help
target\release\pcpulse-tui.exe --version
```

## Project rules

- Do not add unapproved dependencies without asking.
- Missing stats must render as `N/A` with an honest reason.
- Never fake or zero-fill missing sensor values.
- Do not start network/LHM servers without asking for port permission.
- Do not delete files or directories without permission.
- Do not run git write commands unless explicitly requested.
- Use numbered plan files under `plans/`; only `plans/AGENTS.md` is tracked.
