//! Uptime category probe.

use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::add_missing;

use sysinfo::System;

pub fn probe_uptime(snapshot: &mut Snapshot) {
    match System::uptime() {
        Ok(uptime_s) => {
            snapshot.uptime_s = Some(uptime_s);
        },
        Err(_) => {
            add_missing(snapshot, StatId::Uptime, MissReason::CollectorError);
        },
    }
}
