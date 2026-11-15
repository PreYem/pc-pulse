//! Network counter category probe.

use crate::model::{MissReason, Snapshot, StatId};
use crate::registry::{ProbeCtx, add_missing};

pub fn probe_net(snapshot: &mut Snapshot, ctx: &ProbeCtx) {
    let mut count = 0;
    let mut recv = 0_u64;
    let mut sent = 0_u64;
    let mut errors = 0_u64;

    for (_, network) in &ctx.networks {
        count += 1;
        recv += network.total_received();
        sent += network.total_transmitted();
        errors += network.total_errors_on_received();
        errors += network.total_errors_on_transmitted();
    }

    if count == 0 || (recv == 0 && sent == 0 && errors == 0) {
        add_missing(snapshot, StatId::NetIo, MissReason::NoData);
    } else {
        snapshot.net_recv_b = Some(recv);
        snapshot.net_sent_b = Some(sent);
        snapshot.net_errors = Some(errors);
    }
}
