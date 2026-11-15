//! pcpulse-core: cross-platform PC health monitor library.
//!
//! Platform-specific backends live behind `#[cfg(windows)]` / `#[cfg(not
//! windows)]` so the same stats model compiles on Windows, Linux and macOS.
//! Stats are `Option<T>` + `Unavailable(reason)`; anything a platform (or a
//! not-yet-enabled collector) cannot provide renders as `N/A — <reason>`.
//!
//! Windows: sysinfo stats + DXGI GPU identity + opt-in LHM web server.
//! Linux/macOS: safe mode (mature sysinfo backends only); GPU temp/fan/SMART
//! and the experimental `/sys` + hwmon + SMART path are `N/A` until plan #8.

pub mod model;
pub mod registry;
pub mod cpu;
pub mod ram;
pub mod uptime;
pub mod disk;
pub mod net;
pub mod procs;
pub mod gpu;
pub mod lhm;
pub mod live;
pub mod collector;
pub mod health;
pub mod config;
