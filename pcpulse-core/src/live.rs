//! Live collector: one background refresh loop with HOT and COLD cost groups.
//!
//! The TUI (and the future GUI) call `start_live(config)` once, then `update()`
//! per frame. `update()` re-runs the HOT probes (cpu %, per-core, ram, swap, GPU
//! util/temp/power/fan/SMART) only when at least 500 ms has elapsed since the
//! previous HOT tick, and the COLD probes (disks, uptime, network, processes, GPU
//! identity) only when at least ~30 s has elapsed. The UI always renders the last
//! `snapshot`, so a slow cold refresh never blocks the frame and never fakes data.

use crate::config::Config;
use crate::model::{MissReason, Snapshot};
use crate::registry::{ProbeCtx, all_missing, new_probe_ctx, run_cold, run_hot};

use sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
use std::thread::sleep;
use std::time::Instant;

const HOT_MS: u64 = 500;
const COLD_MS: u64 = 30_000;

pub struct LiveCollector {
    pub snapshot: Snapshot,
    ctx: Option<ProbeCtx>,
    config: Config,
    last_hot: Instant,
    last_cold: Instant,
}

pub fn start_live(config: &Config) -> LiveCollector {
    match new_probe_ctx() {
        None => {
            LiveCollector {
                snapshot: all_missing(MissReason::CollectorError),
                ctx: None,
                config: config.clone(),
                last_hot: Instant::now(),
                last_cold: Instant::now(),
            }
        },
        Some(mut ctx) => {
            let mut snapshot = Snapshot::default();

            sleep(MINIMUM_CPU_UPDATE_INTERVAL);

            run_cold(&mut snapshot, &mut ctx, config);
            run_hot(&mut snapshot, &mut ctx, config);

            LiveCollector {
                snapshot,
                ctx: Some(ctx),
                config: config.clone(),
                last_hot: Instant::now(),
                last_cold: Instant::now(),
            }
        },
    }
}

impl LiveCollector {
    pub fn update(&mut self) {
        let now = Instant::now();
        let hot_elapsed = now - self.last_hot;
        let cold_elapsed = now - self.last_cold;
        let need_hot = hot_elapsed.as_millis() as u64 >= HOT_MS;
        let need_cold = cold_elapsed.as_millis() as u64 >= COLD_MS;

        match self.ctx.as_mut() {
            None => {},
            Some(ctx) => {
                if need_hot {
                    run_hot(&mut self.snapshot, ctx, &self.config);
                    self.last_hot = Instant::now();
                }

                if need_cold {
                    run_cold(&mut self.snapshot, ctx, &self.config);
                    self.last_cold = Instant::now();
                }
            },
        }
    }
}
