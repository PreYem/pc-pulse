//! pcpulse-core: cross-platform PC health monitor library.
//!
//! Phase 1: sysinfo-based stats (CPU, RAM, swap, uptime, disks, network,
//! top processes, best-effort temperatures) + health verdict + config.
//! GPU/temp/fan/SMART sensors are N/A with reasons until later phases.

pub mod model;
pub mod sensor;
pub mod collector;
pub mod health;
pub mod config;
