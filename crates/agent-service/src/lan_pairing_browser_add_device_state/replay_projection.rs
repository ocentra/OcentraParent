use super::{scan_history, LanNetworkDeviceScanResult};

pub(crate) fn effective_replay_projection(
    scan_result: &LanNetworkDeviceScanResult,
    persisted_projection: Option<scan_history::LanReplayCanonicalProjection>,
) -> Option<scan_history::LanReplayCanonicalProjection> {
    persisted_projection.or_else(|| {
        scan_result
            .current_scan_snapshot
            .as_ref()
            .and_then(|snapshot| {
                scan_history::valid_replay_projection(snapshot.replay_canonical_projection.as_ref())
                    .cloned()
            })
    })
}
